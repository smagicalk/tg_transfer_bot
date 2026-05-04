// `/job` 控制动作实现。
// 每个动作都先更新数据库状态，再返回可复制的下一步命令按钮。

use crate::tgbot::send;
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::store;
use crate::tgbot::transfer::store::JobProgressSnapshot;
use crate::tgbot::transfer::workflow;

use super::super::common::{
    CommandStyle, downloads_command, format_bytes, job_command as build_job_command,
};
use super::super::downloads::build_downloads_return_list_callback_data;
use super::args::{JobCallbackAction, build_job_callback_data};

/// 暂停任务。
///
/// 当前正在执行的 TDLib 单次下载/上传不会被强制中断；工作流会在下一个安全点停止。
pub(super) async fn pause_job(
    job_id: i64,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let job = store::pause_job(job_id, request_chat_id).await?;
    tracing::info!(
        job_id = job.id,
        request_chat_id,
        status = %job.status,
        "transfer job paused by command"
    );
    send::ReplyPanel::card(format_job_action_text(
        "任务已暂停",
        job.id,
        &job.status,
        "恢复后会从已有子项状态继续处理。",
    ))
    .row(vec![
        send::build_copy_button(
            "复制恢复",
            &build_job_command("r", job.id, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_copy_button(
            "复制停止",
            &build_job_command("s", job.id, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_copy_button(
            "复制暂停列表",
            &downloads_command(Some("pause"), None, None, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .send(request_chat_id, client_id)
    .await
}

/// 唤醒未完成任务。
///
/// paused 会先改回 pending；pending/running 若当前进程没有执行器，也会重新派发后台任务。
pub(super) async fn resume_job(
    job_id: i64,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let job = store::wake_job(job_id, request_chat_id).await?;
    let is_running = workflow::is_job_running_in_process(job.id).await;
    if !is_running {
        super::super::super::spawn_recovery_job(job.clone(), client_id);
    }
    tracing::info!(
        job_id = job.id,
        request_chat_id,
        status = %job.status,
        is_running,
        "transfer job resumed by command"
    );

    let title = if is_running {
        "任务已在执行中"
    } else {
        "任务已唤醒"
    };
    let detail = if is_running {
        "当前进程已有后台执行器，不会重复派发。"
    } else {
        "后台会继续下载/上传剩余内容。"
    };

    send::ReplyPanel::card(format_job_action_text(title, job.id, &job.status, detail))
        .row(vec![
            send::build_copy_button(
                "复制暂停",
                &build_job_command("p", job.id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制停止",
                &build_job_command("s", job.id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制运行列表",
                &downloads_command(Some("run"), None, None, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
        ])
        .send(request_chat_id, client_id)
        .await
}

/// 停止任务。
///
/// 先把数据库状态改成 cancelling，再判断是否存在执行器：
/// - 有执行器：由工作流在安全点释放引用；
/// - 无执行器：当前命令立即收敛为 cancelled 并释放引用。
pub(super) async fn stop_job(
    job_id: i64,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let requested = store::request_cancel_job(job_id, request_chat_id).await?;
    let is_running = workflow::is_job_running_in_process(job_id).await;
    let job = if is_running {
        requested
    } else {
        store::cancel_job_now(
            job_id,
            "cancelled by user",
            super::super::super::runtime_config()
                .file_delete_delay_minutes
                .max(0),
        )
        .await?
    };
    tracing::info!(
        job_id = job.id,
        request_chat_id,
        status = %job.status,
        is_running,
        "transfer job stopped by command"
    );

    let title = if is_running {
        "任务已请求停止"
    } else {
        "任务已停止"
    };
    let detail = if is_running {
        "当前下载/上传调用会在安全点收尾，随后释放文件引用。"
    } else {
        "文件引用已释放，后续由删除队列按配置清理。"
    };

    send::ReplyPanel::card(format_job_action_text(title, job.id, &job.status, detail))
        .row(vec![
            send::build_copy_button(
                "复制 job_id",
                &job.id.to_string(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制停止列表",
                &downloads_command(Some("cancel"), None, None, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .send(request_chat_id, client_id)
        .await
}

/// 查看单个任务详情。
///
/// 只读取轻量进度快照，不会影响后台任务状态。
pub(super) async fn show_job_status(
    job_id: i64,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let Some(snapshot) =
        store::get_job_progress_snapshot_for_request_chat(job_id, request_chat_id).await?
    else {
        anyhow::bail!("job not found: {}", job_id);
    };
    tracing::info!(
        job_id,
        request_chat_id,
        status = %snapshot.job.status,
        "transfer job status requested"
    );

    send::ReplyPanel::card(format_job_status_text(&snapshot))
        .rows(build_job_status_buttons(&snapshot))
        .send(request_chat_id, client_id)
        .await
}

/// 处理 `/job` 详情卡片上的 callback 按钮。
///
/// callback 和文本命令共用同一套状态迁移语义，但 callback 会把当前消息原地编辑成最新详情，
/// 这样用户不需要复制命令，也不会在聊天里刷出多条控制结果。
pub(super) async fn handle_job_callback(
    action: JobCallbackAction,
    job_id: i64,
    request_chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<&'static str> {
    let callback_tip = match action {
        JobCallbackAction::Pause => {
            let job = store::pause_job(job_id, request_chat_id).await?;
            tracing::info!(
                job_id = job.id,
                request_chat_id,
                status = %job.status,
                "transfer job paused by callback"
            );
            "已暂停"
        }
        JobCallbackAction::Resume => {
            let job = store::wake_job(job_id, request_chat_id).await?;
            let is_running = workflow::is_job_running_in_process(job.id).await;
            if !is_running {
                super::super::super::spawn_recovery_job(job.clone(), client_id);
            }
            tracing::info!(
                job_id = job.id,
                request_chat_id,
                status = %job.status,
                is_running,
                "transfer job resumed by callback"
            );
            if is_running {
                "已在执行"
            } else {
                "已恢复"
            }
        }
        JobCallbackAction::Stop => {
            let requested = store::request_cancel_job(job_id, request_chat_id).await?;
            let is_running = workflow::is_job_running_in_process(job_id).await;
            let job = if is_running {
                requested
            } else {
                store::cancel_job_now(
                    job_id,
                    "cancelled by user callback",
                    super::super::super::runtime_config()
                        .file_delete_delay_minutes
                        .max(0),
                )
                .await?
            };
            tracing::info!(
                job_id = job.id,
                request_chat_id,
                status = %job.status,
                is_running,
                "transfer job stopped by callback"
            );
            if is_running {
                "已请求停止"
            } else {
                "已停止"
            }
        }
        JobCallbackAction::Status => {
            tracing::debug!(
                job_id,
                request_chat_id,
                "transfer job status refreshed by callback"
            );
            "已刷新"
        }
    };
    edit_job_status_message(job_id, request_chat_id, message_id, client_id).await?;
    Ok(callback_tip)
}

/// 原地刷新一条任务详情卡片。
///
/// 只读取当前请求聊天可见的任务，避免 callback payload 被复制到其他聊天后越权查看。
async fn edit_job_status_message(
    job_id: i64,
    request_chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let Some(snapshot) =
        store::get_job_progress_snapshot_for_request_chat(job_id, request_chat_id).await?
    else {
        anyhow::bail!("job not found: {}", job_id);
    };
    let (text, keyboard) = send::ReplyPanel::card(format_job_status_text(&snapshot))
        .rows(build_job_status_buttons(&snapshot))
        .into_card_parts()?;
    send::edit_card_message_with_inline_keyboard(
        text,
        request_chat_id,
        message_id,
        keyboard,
        client_id,
    )
    .await
}

/// 构造 `/job` 动作结果卡片。
fn format_job_action_text(title: &str, job_id: i64, status: &str, detail: &str) -> String {
    [
        title.to_owned(),
        format!(
            "job：{}  状态：{}",
            card::job_ref(job_id),
            card::code(status)
        ),
        card::DIVIDER.to_owned(),
        format!("说明：{}", detail),
    ]
    .join("\n")
}

/// 构造单任务详情卡片。
fn format_job_status_text(snapshot: &JobProgressSnapshot) -> String {
    let total = snapshot.job.total_items.max(0);
    let finished = snapshot.success_count + snapshot.failed_count + snapshot.cancelled_count;
    let progress = if total <= 0 {
        0
    } else {
        finished.saturating_mul(100) / total
    };
    let mut lines = vec![
        "任务详情".to_owned(),
        format!(
            "job：{}  状态：{}  目标：{}",
            card::job_ref(snapshot.job.id),
            card::code(&snapshot.job.status),
            card::code(snapshot.job.target_chat_id)
        ),
        card::DIVIDER.to_owned(),
        card::section("进度"),
        format!(
            "总进度：{}",
            card::code(format!("{}/{} ({}%)", finished, total, progress))
        ),
        format!(
            "阶段：等待 {} | 下载 {} | 就绪 {} | 上传 {}",
            card::code(snapshot.pending_count),
            card::code(snapshot.preparing_count),
            card::code(snapshot.prepared_count),
            card::code(snapshot.uploading_count)
        ),
        format!(
            "结果：成功 {} | 失败 {} | 已停 {}",
            card::code(snapshot.success_count),
            card::code(snapshot.failed_count),
            card::code(snapshot.cancelled_count)
        ),
    ];

    if snapshot.active_download_files > 0 {
        lines.push(format!(
            "真实下载：{}",
            card::code(format_job_live_download(snapshot))
        ));
    }

    lines.push(card::section("时间"));
    lines.push(format!(
        "创建：{}",
        card::code(snapshot.job.created_at.format("%Y-%m-%d %H:%M:%S"))
    ));
    lines.push(format!(
        "更新：{}",
        card::code(snapshot.job.updated_at.format("%Y-%m-%d %H:%M:%S"))
    ));
    lines.join("\n")
}

/// 构造单任务详情按钮。
fn build_job_status_buttons(
    snapshot: &JobProgressSnapshot,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let job_id = snapshot.job.id;
    let status = snapshot.job.status.as_str();
    let mut rows = Vec::new();

    if matches!(
        status,
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
    ) {
        rows.push(vec![
            send::build_callback_button(
                "暂停",
                &build_job_callback_data(JobCallbackAction::Pause, job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "停止",
                &build_job_callback_data(JobCallbackAction::Stop, job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ]);
    } else if status == store::JOB_STATUS_PAUSED {
        rows.push(vec![
            send::build_callback_button(
                "恢复",
                &build_job_callback_data(JobCallbackAction::Resume, job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "停止",
                &build_job_callback_data(JobCallbackAction::Stop, job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ]);
    }

    rows.push(vec![
        send::build_callback_button(
            "刷新详情",
            &build_job_callback_data(JobCallbackAction::Status, job_id),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_callback_button(
            "返回列表",
            &build_downloads_return_list_callback_data(status, 8),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_copy_button(
            "复制列表命令",
            &downloads_command(
                Some(job_status_list_filter(status)),
                None,
                None,
                CommandStyle::Short,
            ),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]);
    rows.push(vec![send::build_copy_button(
        "复制详情命令",
        &build_job_command("st", job_id, CommandStyle::Short),
        tdlib_rs::enums::ButtonStyle::Default,
    )]);
    rows
}

/// 根据任务状态选择最接近的 `/downloads` 筛选器。
fn job_status_list_filter(status: &str) -> &'static str {
    match status {
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING => "run",
        store::JOB_STATUS_PAUSED => "pause",
        store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING => "cancelling",
        store::JOB_STATUS_CANCELLED => "cancel",
        store::JOB_STATUS_SUCCESS => "done",
        store::JOB_STATUS_FAILED | store::JOB_STATUS_PARTIAL => "fail",
        _ => "all",
    }
}

/// 渲染单任务详情里的真实下载进度。
fn format_job_live_download(snapshot: &JobProgressSnapshot) -> String {
    let prefix = format!("{} 个文件", snapshot.active_download_files);
    if snapshot.active_download_total_bytes > 0 && !snapshot.has_unknown_download_total {
        let progress = snapshot.active_downloaded_bytes.saturating_mul(100)
            / snapshot.active_download_total_bytes.max(1);
        return format!(
            "{} {}/{} ({}%)",
            prefix,
            format_bytes(snapshot.active_downloaded_bytes),
            format_bytes(snapshot.active_download_total_bytes),
            progress
        );
    }

    if snapshot.active_download_total_bytes > 0 {
        return format!(
            "{} 已下 {} / 已知总量 {}+",
            prefix,
            format_bytes(snapshot.active_downloaded_bytes),
            format_bytes(snapshot.active_download_total_bytes)
        );
    }

    format!(
        "{} 已下 {}",
        prefix,
        format_bytes(snapshot.active_downloaded_bytes)
    )
}

#[cfg(test)]
mod tests {
    use super::{
        build_job_status_buttons, format_job_action_text, format_job_live_download,
        format_job_status_text, job_status_list_filter,
    };
    use crate::tgbot::transfer::store;

    // job 控制回复应使用 card 代码字段展示 job_id 和状态。
    #[test]
    fn test_format_job_action_text() {
        let text = format_job_action_text("任务已暂停", 42, "paused", "等待恢复。");

        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("状态：‹paused›"));
        assert!(text.contains("说明：等待恢复。"));
    }

    // 单任务详情应展示状态、目标、进度和时间字段。
    #[test]
    fn test_format_job_status_text() {
        let snapshot = snapshot_with_status(store::JOB_STATUS_RUNNING);
        let text = format_job_status_text(&snapshot);

        assert!(text.contains("任务详情"));
        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("状态：‹running›"));
        assert!(text.contains("总进度：‹1/3 (33%)›"));
        assert!(text.contains("■ 时间"));
    }

    // 运行中任务详情应提供暂停/停止 callback 按钮，便于直接控制。
    #[test]
    fn test_build_job_status_buttons_for_running() {
        let buttons = build_job_status_buttons(&snapshot_with_status(store::JOB_STATUS_RUNNING));

        assert_eq!(buttons[0][0].text, "暂停");
        assert_eq!(buttons[0][1].text, "停止");
        assert_eq!(buttons[1][0].text, "刷新详情");
        assert!(matches!(
            buttons[0][0].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
    }

    // paused 任务详情应提供恢复 callback 按钮。
    #[test]
    fn test_build_job_status_buttons_for_paused() {
        let buttons = build_job_status_buttons(&snapshot_with_status(store::JOB_STATUS_PAUSED));

        assert_eq!(buttons[0][0].text, "恢复");
        assert_eq!(buttons[0][1].text, "停止");
    }

    // 任务详情里的返回列表按钮应直接回到对应的 downloads 筛选入口。
    #[test]
    fn test_build_job_status_buttons_has_return_list_button() {
        let buttons = build_job_status_buttons(&snapshot_with_status(store::JOB_STATUS_RUNNING));

        assert_eq!(buttons[1][1].text, "返回列表");
        assert!(matches!(
            buttons[1][1].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
    }

    // 任务状态应映射到最接近的 downloads 筛选。
    #[test]
    fn test_job_status_list_filter() {
        assert_eq!(job_status_list_filter(store::JOB_STATUS_RUNNING), "run");
        assert_eq!(job_status_list_filter(store::JOB_STATUS_PAUSED), "pause");
        assert_eq!(job_status_list_filter(store::JOB_STATUS_SUCCESS), "done");
        assert_eq!(job_status_list_filter(store::JOB_STATUS_FAILED), "fail");
    }

    // 真实下载摘要应和下载列表保持同一风格。
    #[test]
    fn test_format_job_live_download() {
        let mut snapshot = snapshot_with_status(store::JOB_STATUS_RUNNING);
        snapshot.active_download_files = 1;
        snapshot.active_downloaded_bytes = 1024;
        snapshot.active_download_total_bytes = 2048;

        assert_eq!(
            format_job_live_download(&snapshot),
            "1 个文件 1.0 KB/2.0 KB (50%)"
        );
    }

    fn snapshot_with_status(status: &str) -> store::JobProgressSnapshot {
        let now = store::now_utc8();
        store::JobProgressSnapshot {
            job: store::JobProgressJob {
                id: 42,
                target_chat_id: -100,
                status: status.to_owned(),
                total_items: 3,
                created_at: now,
                updated_at: now,
            },
            pending_count: 1,
            preparing_count: 0,
            prepared_count: 1,
            uploading_count: 0,
            success_count: 1,
            failed_count: 0,
            cancelled_count: 0,
            active_download_files: 0,
            active_downloaded_bytes: 0,
            active_download_total_bytes: 0,
            has_unknown_download_total: false,
        }
    }
}
