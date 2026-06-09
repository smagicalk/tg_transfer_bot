// 转存恢复流程。
// 启动时恢复数据库中的 pending/running 任务，并收敛重启前已经 cancelling 的任务。

use crate::db;
use sea_orm::EntityTrait;

use super::super::card;
use super::super::command::{build_downloads_short_command, build_downloads_status_button_data};
use super::super::types::SourceKind;
use super::super::{spider, store};
use super::TransferOutcome;
use super::control::{apply_job_control, finish_skipped_by_control};
use super::guard::acquire_job_guard;
use super::runner::run_job_inner;

/// 启动时恢复数据库里未完成任务。
pub(in crate::tgbot::transfer) async fn recover_unfinished_jobs(
    client_ids: crate::config::TransferClientIds,
) -> anyhow::Result<()> {
    // 上次退出前已经请求停止的任务，启动时先收敛为 cancelled 并释放引用。
    let cancelling_jobs = store::list_cancelling_jobs().await?;
    let cancelling_count = cancelling_jobs.len();
    let mut summaries = RecoveryStartupSummaries::default();
    for job in cancelling_jobs {
        tracing::info!(
            job_id = job.id,
            request_chat_id = job.request_chat_id,
            target_chat_id = job.target_chat_id,
            status = %job.status,
            "finalize cancelling transfer job after restart"
        );
        store::cancel_job_now(
            job.id,
            "cancelled by user before restart",
            super::file_delete_delay_minutes(),
        )
        .await?;
        summaries.add_finalized(&job);
    }

    let jobs = store::list_recoverable_jobs().await?;
    tracing::info!(
        cancelling_finalized_count = cancelling_count,
        recoverable_count = jobs.len(),
        "transfer recovery scan completed"
    );
    if jobs.is_empty() {
        tracing::info!("no recoverable transfer jobs");
        summaries.send(client_ids.interaction).await;
        return Ok(());
    }

    tracing::info!(
        recoverable_count = jobs.len(),
        "scheduling unfinished transfer jobs"
    );
    for job in jobs {
        tracing::info!(
            job_id = job.id,
            request_chat_id = job.request_chat_id,
            target_chat_id = job.target_chat_id,
            status = %job.status,
            "schedule recover job"
        );
        summaries.add_recoverable(&job);
        super::super::spawn_recovery_job(job, client_ids);
    }
    summaries.send(client_ids.interaction).await;
    Ok(())
}

/// 恢复单个任务：
/// - 重新抓取 source_link
/// - 对齐子项并执行
pub(in crate::tgbot::transfer) async fn resume_one_job(
    job: db::transfer_job::Model,
    client_ids: crate::config::TransferClientIds,
) -> anyhow::Result<TransferOutcome> {
    // 恢复流程从抓取源消息开始就占用 job 运行锁，避免 stop 命令误判“无执行器”后直接释放引用。
    let _guard = match acquire_job_guard(job.id).await {
        Some(g) => g,
        None => {
            tracing::info!(
                job_id = job.id,
                request_chat_id = job.request_chat_id,
                target_chat_id = job.target_chat_id,
                "recovery skipped because job is already running"
            );
            return Ok(TransferOutcome::Running { job_id: job.id });
        }
    };

    if let Some(outcome) = apply_job_control(job.id).await? {
        tracing::info!(
            job_id = job.id,
            request_chat_id = job.request_chat_id,
            target_chat_id = job.target_chat_id,
            "recovery stopped by control before spider"
        );
        return Ok(outcome);
    }

    tracing::info!(
        job_id = job.id,
        request_chat_id = job.request_chat_id,
        target_chat_id = job.target_chat_id,
        "recovery spider started"
    );
    let source_kind = SourceKind::from_str(&job.source_kind)
        .ok_or_else(|| anyhow::anyhow!("invalid source_kind: {}", job.source_kind))?;
    let bundle = match source_kind {
        SourceKind::Link => {
            if client_ids.bot.is_some() {
                spider::spider_link_bot_first(
                    job.source_link.clone(),
                    client_ids.get(crate::config::ClientRole::Bot)?,
                    client_ids.get(crate::config::ClientRole::User)?,
                )
                .await?
            } else {
                spider::spider_message(
                    job.source_link.clone(),
                    client_ids.get(crate::config::ClientRole::User)?,
                    crate::config::ClientRole::User,
                )
                .await?
            }
        }
        SourceKind::BotMessage => {
            spider::spider_bot_visible_message(
                job.source_chat_id,
                job.source_message_id,
                client_ids.get(crate::config::ClientRole::Bot)?,
            )
            .await?
        }
    };
    if let Some(outcome) = apply_job_control(job.id).await? {
        tracing::info!(
            job_id = job.id,
            request_chat_id = job.request_chat_id,
            target_chat_id = job.target_chat_id,
            "recovery stopped by control after spider"
        );
        return Ok(outcome);
    }

    if !store::mark_job_running(job.id).await? {
        tracing::info!(
            job_id = job.id,
            request_chat_id = job.request_chat_id,
            target_chat_id = job.target_chat_id,
            "recovery mark running skipped by control"
        );
        return finish_skipped_by_control(job.id).await;
    }
    tracing::info!(
        job_id = job.id,
        source_chat_id = bundle.source_chat_id,
        source_message_id = bundle.source_message_id,
        source_album_id = bundle.source_album_id,
        message_count = bundle.messages.len(),
        "recovery job marked running"
    );
    // 恢复时以重新 spider 到的链接内容为准，并同步修正旧 item/file_cache 引用：
    // 新出现的消息会新增，消失的旧消息会 obsolete，文件变化的消息会迁移 file_key。
    store::reconcile_items_for_bundle(job.id, &bundle, super::file_delete_delay_minutes()).await?;
    let refreshed_job = db::transfer_job::Entity::find_by_id(job.id)
        .one(crate::db::get_db().await?)
        .await?
        .ok_or_else(|| anyhow::anyhow!("transfer job disappeared during recovery: {}", job.id))?;
    run_job_inner(refreshed_job, bundle.messages, client_ids).await
}

/// 启动恢复摘要，按原请求 chat 聚合。
///
/// 恢复任务本身仍逐个派发；摘要只负责让用户在启动后立刻知道后台做了什么。
#[derive(Default)]
struct RecoveryStartupSummaries {
    by_chat: std::collections::BTreeMap<i64, RecoveryStartupSummary>,
}

#[derive(Default)]
struct RecoveryStartupSummary {
    recoverable_count: usize,
    finalized_count: usize,
    bot_source_count: usize,
    user_source_count: usize,
    sample_job_ids: Vec<i64>,
}

impl RecoveryStartupSummaries {
    /// 记录一个已收敛的取消任务。
    fn add_finalized(&mut self, job: &db::transfer_job::Model) {
        self.entry(job.request_chat_id).finalized_count += 1;
    }

    /// 记录一个将被恢复执行的任务。
    fn add_recoverable(&mut self, job: &db::transfer_job::Model) {
        let summary = self.entry(job.request_chat_id);
        summary.recoverable_count += 1;
        match job.source_client_role.as_str() {
            "bot" => summary.bot_source_count += 1,
            "user" => summary.user_source_count += 1,
            _ => {}
        }
        if summary.sample_job_ids.len() < 5 {
            summary.sample_job_ids.push(job.id);
        }
    }

    /// 按请求 chat 发送恢复摘要。
    async fn send(self, client_id: i32) {
        for (chat_id, summary) in self.by_chat {
            if !summary.should_send() {
                continue;
            }
            let panel =
                crate::tgbot::send::ReplyPanel::card(format_recovery_startup_text(&summary))
                    .row(vec![
                        crate::tgbot::send::build_callback_button(
                            "运行列表",
                            &build_downloads_status_button_data("running", 8),
                            tdlib_rs::enums::ButtonStyle::Primary,
                        ),
                        crate::tgbot::send::build_callback_button(
                            "暂停列表",
                            &build_downloads_status_button_data("paused", 8),
                            tdlib_rs::enums::ButtonStyle::Default,
                        ),
                        crate::tgbot::send::build_callback_button(
                            "停止列表",
                            &build_downloads_status_button_data("cancelled", 8),
                            tdlib_rs::enums::ButtonStyle::Default,
                        ),
                    ])
                    .row(vec![
                        crate::tgbot::send::build_callback_button(
                            "全部任务",
                            &build_downloads_status_button_data("all", 8),
                            tdlib_rs::enums::ButtonStyle::Default,
                        ),
                        crate::tgbot::send::build_copy_button(
                            "复制运行命令",
                            &build_downloads_short_command(Some("run")),
                            tdlib_rs::enums::ButtonStyle::Default,
                        ),
                    ]);
            if let Err(err) = panel.send(chat_id, client_id).await {
                tracing::warn!(chat_id, error = %err, "send recovery startup summary failed");
            }
        }
    }

    fn entry(&mut self, request_chat_id: i64) -> &mut RecoveryStartupSummary {
        self.by_chat.entry(request_chat_id).or_default()
    }
}

impl RecoveryStartupSummary {
    fn should_send(&self) -> bool {
        self.recoverable_count > 0 || self.finalized_count > 0
    }
}

/// 构造启动恢复摘要卡片。
fn format_recovery_startup_text(summary: &RecoveryStartupSummary) -> String {
    let sample_jobs = if summary.sample_job_ids.is_empty() {
        "无".to_owned()
    } else {
        summary
            .sample_job_ids
            .iter()
            .map(|id| format!("#{}", id))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let note = if summary.recoverable_count > 0 {
        "后台已自动派发可恢复任务，可用按钮查看当前运行列表。"
    } else {
        "没有需要重新执行的任务，本次只完成了重启前停止任务的收敛。"
    };
    [
        "启动恢复摘要".to_owned(),
        card::field_pair(
            "恢复中",
            summary.recoverable_count,
            "已收敛停止",
            summary.finalized_count,
        ),
        card::field_pair(
            "bot源",
            summary.bot_source_count,
            "user源",
            summary.user_source_count,
        ),
        card::DIVIDER.to_owned(),
        card::section("任务"),
        format!("示例 job：{}", card::code(sample_jobs)),
        card::note(note),
        card::section("命令"),
        card::command_line("运行列表", build_downloads_short_command(Some("run"))),
        card::command_line("暂停列表", build_downloads_short_command(Some("pause"))),
        card::command_line("全部任务", build_downloads_short_command(Some("all"))),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{RecoveryStartupSummary, format_recovery_startup_text};

    // 启动恢复摘要应展示恢复数量、收敛数量和示例 job，便于启动后快速排查。
    #[test]
    fn test_format_recovery_startup_text() {
        let summary = RecoveryStartupSummary {
            recoverable_count: 2,
            finalized_count: 1,
            bot_source_count: 1,
            user_source_count: 1,
            sample_job_ids: vec![11, 12],
        };

        let text = format_recovery_startup_text(&summary);

        assert!(text.contains("启动恢复摘要"));
        assert!(text.contains("恢复中：‹2›"));
        assert!(text.contains("已收敛停止：‹1›"));
        assert!(text.contains("bot源：‹1›"));
        assert!(text.contains("user源：‹1›"));
        assert!(text.contains("示例 job：‹#11, #12›"));
        assert!(text.contains("运行列表：‹/d run›"));
    }

    // 只有停止任务被收敛时，摘要不应误导用户“后台已派发恢复任务”。
    #[test]
    fn test_format_recovery_startup_text_for_finalized_only() {
        let summary = RecoveryStartupSummary {
            recoverable_count: 0,
            finalized_count: 2,
            bot_source_count: 0,
            user_source_count: 0,
            sample_job_ids: vec![],
        };

        let text = format_recovery_startup_text(&summary);

        assert!(text.contains("恢复中：‹0›"));
        assert!(text.contains("示例 job：‹无›"));
        assert!(text.contains("没有需要重新执行的任务"));
        assert!(!text.contains("已自动派发可恢复任务"));
    }
}
