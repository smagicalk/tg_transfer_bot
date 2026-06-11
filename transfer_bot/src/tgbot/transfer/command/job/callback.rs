// `/job` inline callback 处理。
// callback 与文本命令共享状态迁移语义，但会原地编辑当前任务详情卡片。

use crate::tgbot::send;
use crate::tgbot::transfer::store;
use crate::tgbot::transfer::workflow;

use super::args::JobCallbackAction;
use super::keyboard::build_job_status_buttons;
use super::render::format_job_status_text;

/// 处理 `/job` 详情卡片上的 callback 按钮。
///
/// callback 和文本命令共用同一套状态迁移语义，但 callback 会把当前消息原地编辑成最新详情，
/// 这样用户不需要复制命令，也不会在聊天里刷出多条控制结果。
pub(super) async fn handle_job_callback(
    action: JobCallbackAction,
    job_id: i64,
    actor: crate::config::RequestActor,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<&'static str> {
    let callback_tip = match action {
        JobCallbackAction::Pause => {
            let job = store::pause_job_with_owner_scope(
                job_id,
                actor.request_chat_id,
                actor.owner_scope(),
            )
            .await?;
            tracing::info!(
                job_id = job.id,
                request_chat_id = actor.request_chat_id,
                owner_user_id = actor.user_id,
                actor_role = actor.role.as_str(),
                status = %job.status,
                "transfer job paused by callback"
            );
            "已暂停"
        }
        JobCallbackAction::Resume => {
            let job = store::wake_job_with_owner_scope(
                job_id,
                actor.request_chat_id,
                actor.owner_scope(),
            )
            .await?;
            let is_running = workflow::is_job_running_in_process(job.id).await;
            if !is_running {
                super::super::super::spawn_recovery_job(
                    crate::app_context::app_context(),
                    job.clone(),
                    super::super::super::transfer_client_ids()?,
                );
            }
            tracing::info!(
                job_id = job.id,
                request_chat_id = actor.request_chat_id,
                owner_user_id = actor.user_id,
                actor_role = actor.role.as_str(),
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
            let requested = store::request_cancel_job_with_owner_scope(
                job_id,
                actor.request_chat_id,
                actor.owner_scope(),
            )
            .await?;
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
                request_chat_id = actor.request_chat_id,
                owner_user_id = actor.user_id,
                actor_role = actor.role.as_str(),
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
                request_chat_id = actor.request_chat_id,
                owner_user_id = actor.user_id,
                actor_role = actor.role.as_str(),
                "transfer job status refreshed by callback"
            );
            "已刷新"
        }
    };
    edit_job_status_message(job_id, actor, message_id, client_id).await?;
    Ok(callback_tip)
}

/// 原地刷新一条任务详情卡片。
///
/// 只读取当前请求聊天可见的任务，避免 callback payload 被复制到其他聊天后越权查看。
async fn edit_job_status_message(
    job_id: i64,
    actor: crate::config::RequestActor,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let Some(snapshot) = store::get_job_progress_snapshot_for_actor(job_id, actor).await? else {
        anyhow::bail!("job not found: {}", job_id);
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
