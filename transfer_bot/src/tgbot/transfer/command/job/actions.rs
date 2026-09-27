// `/job` 控制动作实现。
// 每个动作都先更新数据库状态，再返回可操作的下一步命令按钮。

use crate::tgbot::send;
use crate::tgbot::transfer::store;
use crate::tgbot::transfer::workflow;

use super::super::{build_downloads_filter_button_data, build_menu_home_button_data};
use super::keyboard::build_job_status_buttons;
use super::render::{format_job_action_text, format_job_status_text};
use super::{
    build_job_pause_callback_data, build_job_resume_callback_data, build_job_status_callback_data,
    build_job_stop_callback_data,
};

/// 选取最佳可点击的结果消息定位链接。
///
/// 优先选择当前链接或明细记录中有效的 HTTP/HTTPS 可跳转 URL。
///
/// # 参数
/// - `current_link`: 任务主表中当前保存的 result_message_link
/// - `records`: 该任务所有子消息的结果记录切片
fn preferred_job_result_link(
    current_link: Option<&str>,
    records: &[store::ResultMessageRecord],
) -> Option<String> {
    current_link
        // 若当前主链接就是可打开的 URL 则优先使用
        .filter(|link| send::is_openable_url(link))
        // 否则从所有结果记录中查找首个可打开的 URL
        .or_else(|| {
            records
                .iter()
                .map(|record| record.message_link.as_str())
                .find(|link| send::is_openable_url(link))
        })
        // 再次兜底回当前主链接（哪怕是普通定位符）
        .or(current_link)
        // 最后兜底使用首个结果记录的定位符
        .or_else(|| records.first().map(|record| record.message_link.as_str()))
        .map(str::to_owned)
}

/// 读取任务详情前刷新历史定位链接，确保超级群目标能提供可点击地址。
///
/// # 参数
/// - `app`: 全局应用上下文实例引用
/// - `job_id`: 目标任务主键 ID
pub(super) async fn load_job_status_snapshot(
    app: &crate::app_context::AppContext,
    job_id: i64,
) -> anyhow::Result<Option<store::JobProgressSnapshot>> {
    // 从底层存储查询带进度的任务快照
    let Some(mut snapshot) = store::get_job_progress_snapshot_with_context(app, job_id).await?
    else {
        return Ok(None);
    };

    // 若当前主链接不是合法的可打开 URL，则尝试从历史消息记录中解析与刷新
    if snapshot
        .job
        .result_message_link
        .as_deref()
        .is_none_or(|link| !send::is_openable_url(link))
    {
        let mut records = store::list_result_messages_by_job(job_id).await?;
        if !records.is_empty() {
            // 获取上传端客户端 ID 尝试向 TDLib 刷新最新超级群公网链接
            match super::super::super::transfer_client_ids() {
                Ok(client_ids) => {
                    match workflow::refresh_stored_result_messages(
                        job_id,
                        records,
                        client_ids.upload,
                    )
                    .await
                    {
                        Ok(refreshed) => records = refreshed,
                        Err(err) => {
                            tracing::warn!(job_id, error = %err, "refresh job result links failed");
                            records = store::list_result_messages_by_job(job_id).await?;
                        }
                    }
                }
                Err(err) => {
                    tracing::warn!(job_id, error = %err, "upload client unavailable while refreshing job result links");
                }
            }

            // 计算最优选链接
            let preferred =
                preferred_job_result_link(snapshot.job.result_message_link.as_deref(), &records);
            // 若最优链接与快照中现有不同，则回写数据库并同步快照
            if preferred != snapshot.job.result_message_link {
                if let Some(link) = preferred.as_ref()
                    && send::is_openable_url(link)
                    && let Err(err) = store::update_result_message_link(job_id, link.clone()).await
                {
                    tracing::warn!(job_id, error = %err, "sync primary job result link failed");
                }
                snapshot.job.result_message_link = preferred;
            }
        }
    }
    Ok(Some(snapshot))
}

/// 获取当前配置的文件删除延迟时间（以分钟为单位，最小为 0）。
///
/// # 参数
/// - `app`: 全局应用上下文实例引用
fn file_delete_delay_minutes_on(app: &crate::app_context::AppContext) -> i64 {
    crate::tgbot::transfer::runtime_config_on(app)
        .file_delete_delay_minutes
        .max(0)
}

/// 在指定应用上下文上执行任务暂停操作。
///
/// # 参数
/// - `_app`: 应用上下文引用
/// - `job_id`: 目标任务 ID
/// - `actor`: 发起操作的用户身份
/// - `client_id`: 响应的 TDLib 客户端实例 ID
pub(super) async fn pause_job_on(
    _app: &crate::app_context::AppContext,
    job_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    // 1. 将数据库中任务状态置为 paused
    let job = store::pause_job(job_id).await?;
    tracing::info!(
        job_id = job.id,
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        status = %job.status,
        "transfer job paused by command"
    );

    // 2. 组装操作反馈卡片并发送给用户
    send::ReplyPanel::card(format_job_action_text(
        "任务已暂停",
        job.id,
        &job.status,
        "恢复后会从已有子项状态继续处理。",
    ))
    .rows(build_pause_job_action_rows(job.id))
    .send(actor.request_chat_id, client_id)
    .await
}

/// 构造暂停结果卡片的下一步操作按钮行。
///
/// 暂停后的下一步都是明确 callback；正文命令已经能兜底，这里不再重复复制 `job_id`。
///
/// # 参数
/// - `job_id`: 任务 ID
fn build_pause_job_action_rows(job_id: i64) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        // 第一行：查看详情、恢复执行、彻底停止
        vec![
            send::build_callback_button(
                "查看详情",
                &build_job_status_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "恢复",
                &build_job_resume_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "停止",
                &build_job_stop_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
        ],
        // 第二行：查看暂停列表、返回菜单
        vec![
            send::build_callback_button(
                "查看暂停列表",
                &build_downloads_filter_button_data("pause", 8).expect("pause filter should exist"),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "菜单",
                &build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
    ]
}

/// 在指定应用上下文上唤醒并恢复未完成的任务。
///
/// # 参数
/// - `app`: 全局应用上下文实例引用
/// - `job_id`: 目标任务 ID
/// - `actor`: 请求发起者身份
/// - `client_id`: TDLib 客户端实例 ID
pub(super) async fn resume_job_on(
    app: &crate::app_context::AppContext,
    job_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    // 1. 唤醒任务在数据库中的状态（置为 pending/running）
    let job = store::wake_job(job_id).await?;
    // 恢复任务最终需要把后台执行器派发到 tokio 中，因此这里把当前请求的
    // `&AppContext` 克隆成 `Arc<AppContext>`，保持执行器和当前运行态一致。
    let app_context = std::sync::Arc::new(app.clone());
    // 检查该任务是否已在当前进程内存运行中
    let is_running = workflow::is_job_running_in_process(app, job.id).await;
    if !is_running {
        // 未在运行，派发新的恢复执行后台协程
        super::super::super::spawn_recovery_job(
            app_context,
            job.clone(),
            super::super::super::transfer_client_ids()?,
            None,
        );
    }
    tracing::info!(
        job_id = job.id,
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        status = %job.status,
        is_running,
        "transfer job resumed by command"
    );

    // 区分已在运行还是新唤醒的文案提示
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

    // 组装并发送反馈卡片
    send::ReplyPanel::card(format_job_action_text(title, job.id, &job.status, detail))
        .row(vec![
            send::build_callback_button(
                "查看详情",
                &build_job_status_callback_data(job.id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "暂停",
                &build_job_pause_callback_data(job.id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "停止",
                &build_job_stop_callback_data(job.id),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
        ])
        .row(vec![
            send::build_callback_button(
                "查看运行列表",
                &build_downloads_filter_button_data("run", 8).expect("run filter should exist"),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "菜单",
                &build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .send(actor.request_chat_id, client_id)
        .await
}

/// 在指定应用上下文上请求彻底停止任务。
///
/// # 参数
/// - `app`: 全局应用上下文实例引用
/// - `job_id`: 目标任务 ID
/// - `actor`: 请求发起者身份
/// - `client_id`: TDLib 客户端实例 ID
pub(super) async fn stop_job_on(
    app: &crate::app_context::AppContext,
    job_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    // 标记任务为请求取消
    let requested = store::request_cancel_job(job_id).await?;
    let is_running = workflow::is_job_running_in_process(app, job_id).await;
    // 若在运行中则等待安全点收尾；若不在运行中则立即执行彻底取消并排队清理文件
    let job = if is_running {
        requested
    } else {
        store::cancel_job_now(
            job_id,
            "cancelled by user",
            file_delete_delay_minutes_on(app),
        )
        .await?
    };
    tracing::info!(
        job_id = job.id,
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
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

    // 组装并发送反馈卡片
    send::ReplyPanel::card(format_job_action_text(title, job.id, &job.status, detail))
        .row(vec![
            send::build_callback_button(
                "查看详情",
                &build_job_status_callback_data(job.id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "查看已停列表",
                &build_downloads_filter_button_data("cancel", 8)
                    .expect("cancel filter should exist"),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "菜单",
                &build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .send(actor.request_chat_id, client_id)
        .await
}

/// 在指定应用上下文上展示单个任务的完整状态与进度详情。
///
/// # 参数
/// - `app`: 全局应用上下文实例引用
/// - `job_id`: 目标任务 ID
/// - `actor`: 请求发起者身份
/// - `client_id`: TDLib 客户端实例 ID
pub(super) async fn show_job_status_on(
    app: &crate::app_context::AppContext,
    job_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    // 拉取最新的任务进度快照
    let Some(snapshot) = load_job_status_snapshot(app, job_id).await? else {
        anyhow::bail!("job not found: {job_id}");
    };
    tracing::info!(
        job_id,
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        status = %snapshot.job.status,
        "transfer job status requested"
    );

    // 渲染卡片正文及配套按钮行并发送
    send::ReplyPanel::card(format_job_status_text(&snapshot))
        .rows(build_job_status_buttons(&snapshot))
        .send(actor.request_chat_id, client_id)
        .await
}

#[cfg(test)]
mod tests {
    use super::{build_pause_job_action_rows, preferred_job_result_link};
    use crate::tgbot::transfer::store::ResultMessageRecord;
    use base64::{Engine as _, engine::general_purpose};

    /// 验证暂停结果卡片提供直接操作按钮，且停止按钮为 Danger 样式回调。
    #[test]
    fn test_build_pause_job_action_rows() {
        let rows = build_pause_job_action_rows(42);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(rows[0][0].text, "查看详情");
        assert_eq!(rows[0][1].text, "恢复");
        assert_eq!(rows[0][2].text, "停止");
        assert_eq!(rows[0][2].style, tdlib_rs::enums::ButtonStyle::Danger);
        assert_eq!(decoded_callback_data(&rows[0][2]), "j:sc:42");
        assert_eq!(rows[1][0].text, "查看暂停列表");
        assert_eq!(rows[1][1].text, "菜单");
        assert_eq!(rows.len(), 2);
        assert!(!labels.contains(&"复制停止命令"));
        assert!(!labels.contains(&"复制 job_id"));
        assert!(matches!(
            rows[0][1].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
    }

    /// 主任务字段为空但结果明细已有 URL 时，详情必须采用明细地址生成跳转入口。
    #[test]
    fn test_preferred_job_result_link_uses_first_openable_result_record() {
        let records = vec![ResultMessageRecord {
            result_index: 0,
            target_chat_id: -100123,
            message_id: 734003200,
            message_link: "https://t.me/c/123/700".to_owned(),
            is_album: false,
            item_count: 1,
        }];

        assert_eq!(
            preferred_job_result_link(None, &records).as_deref(),
            Some("https://t.me/c/123/700")
        );
    }

    /// 测试辅助工具：从按钮中解码出原始 callback payload 字符串。
    fn decoded_callback_data(button: &tdlib_rs::types::InlineKeyboardButton) -> String {
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &button.r#type else {
            panic!("button must be callback");
        };
        String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap()
    }
}
