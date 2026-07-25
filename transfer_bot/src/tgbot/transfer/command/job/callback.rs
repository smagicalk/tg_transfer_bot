// `/job` inline callback 处理。
// callback 与文本命令共享状态迁移语义，但会原地编辑当前任务详情卡片。

use crate::tgbot::send;
use crate::tgbot::transfer::store;
use crate::tgbot::transfer::workflow;

use super::actions::load_job_status_snapshot;
use super::args::JobCallbackAction;
use super::keyboard::build_job_status_buttons;
use super::render::format_job_status_text;

/// 当前文件删除延迟（分钟）。
fn file_delete_delay_minutes_on(app: &crate::app_context::AppContext) -> i64 {
    crate::tgbot::transfer::runtime_config_on(app)
        .file_delete_delay_minutes
        .max(0)
}

/// job callback 的处理结果。
///
/// 状态迁移成功但消息编辑失败时，不能再把错误描述成“任务操作失败”，否则用户会误以为
/// pause/resume/stop 没有生效。这里把刷新失败单独交给入口层发送更准确的提示。
pub(super) enum JobCallbackResult {
    /// 状态操作和详情刷新都完成。
    Updated,
    /// 状态操作已完成，但详情卡片没有刷新成功。
    RefreshFailed(anyhow::Error),
}

/// 处理 `/job` 详情卡片上的 callback 按钮。
///
/// callback 和文本命令共用同一套状态迁移语义，但 callback 会把当前消息原地编辑成最新详情，
/// 这样用户不需要复制命令，也不会在聊天里刷出多条控制结果。
pub(super) async fn handle_job_callback(
    app: &crate::app_context::AppContext,
    action: JobCallbackAction,
    job_id: i64,
    actor: crate::config::RequestActor,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<JobCallbackResult> {
    match action {
        JobCallbackAction::Pause => {
            let job = store::pause_job(job_id).await?;
            tracing::info!(
                job_id = job.id,
                request_chat_id = actor.request_chat_id,
                owner_user_id = actor.user_id,
                owner_user_id = actor.user_id,
                status = %job.status,
                "transfer job paused by callback"
            );
        }
        JobCallbackAction::Resume => {
            let job = store::wake_job(job_id).await?;
            let is_running = workflow::is_job_running_in_process(app, job.id).await;
            if !is_running {
                // callback 链沿用当前请求的 `&AppContext`，再克隆成 `Arc<AppContext>`
                // 后台派发即可，避免恢复链再回退到全局单例。
                let app_context = std::sync::Arc::new(app.clone());
                super::super::super::spawn_recovery_job(
                    app_context,
                    job.clone(),
                    super::super::super::transfer_client_ids()?,
                    Some(message_id),
                );
            }
            tracing::info!(
                job_id = job.id,
                request_chat_id = actor.request_chat_id,
                owner_user_id = actor.user_id,
                owner_user_id = actor.user_id,
                status = %job.status,
                is_running,
                "transfer job resumed by callback"
            );
        }
        // 旧版本的 StopConfirm callback 仍会出现在历史消息中；直接执行停止，
        // 避免进度刷新器覆盖确认页导致用户永远无法完成第二次点击。
        JobCallbackAction::StopConfirm | JobCallbackAction::Stop => {
            let requested = store::request_cancel_job(job_id).await?;
            let is_running = workflow::is_job_running_in_process(app, job_id).await;
            let job = if is_running {
                requested
            } else {
                store::cancel_job_now(
                    job_id,
                    "cancelled by user callback",
                    file_delete_delay_minutes_on(app),
                )
                .await?
            };
            tracing::info!(
                job_id = job.id,
                request_chat_id = actor.request_chat_id,
                owner_user_id = actor.user_id,
                owner_user_id = actor.user_id,
                status = %job.status,
                is_running,
                "transfer job stopped by callback"
            );
        }
        JobCallbackAction::Status => {
            tracing::debug!(
                job_id,
                request_chat_id = actor.request_chat_id,
                owner_user_id = actor.user_id,
                owner_user_id = actor.user_id,
                "transfer job status refreshed by callback"
            );
        }
    };
    if let Err(err) = edit_job_status_message(app, job_id, actor, message_id, client_id).await {
        return Ok(JobCallbackResult::RefreshFailed(err));
    }
    Ok(JobCallbackResult::Updated)
}

/// 原地刷新一条任务详情卡片。
///
/// 只读取当前请求聊天可见的任务，避免 callback payload 被复制到其他聊天后越权查看。
async fn edit_job_status_message(
    app: &crate::app_context::AppContext,
    job_id: i64,
    actor: crate::config::RequestActor,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let Some(snapshot) = load_job_status_snapshot(app, job_id).await? else {
        anyhow::bail!("job not found: {job_id}");
    };
    let (text, keyboard) = send::ReplyPanel::card(format_job_status_text(&snapshot))
        .rows(build_job_status_buttons(&snapshot))
        .into_card_parts()?;
    send::edit_card_message_with_inline_keyboard(
        text,
        actor.request_chat_id,
        message_id,
        keyboard,
        client_id,
    )
    .await
}
