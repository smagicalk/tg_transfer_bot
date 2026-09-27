// 转存后台任务派发。
// 命令入口只负责快速回复；真正下载、上传、恢复任务都在这里通过 tokio 后台执行。

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::progress::{
    RecoveryProgressUpdate, edit_transfer_progress_for_outcome, update_recovery_progress_message,
    update_transfer_progress_message,
};
use super::{types, workflow};

mod result;

use result::{send_recovery_outcome, send_transfer_outcome};

/// 派发新的 `/transfer` 后台任务。
///
/// 行为与生命周期说明：
/// 1. 准入控制：通过 `TransferAdmissionGuard` 保证系统未处于 draining/停止状态。
/// 2. 进度上报：若提供了 `progress_message_id`，启动后台异步轮询协程周期性编辑该消息，实时展示下载/上传百分比。
/// 3. 并发插槽：异步排队获取 `acquire_transfer_slot()`，避免瞬时并发耗尽带宽或内存。
/// 4. 核心工作流：委托给 `workflow::transfer` 执行下载、缓存与发送。
/// 5. 结果交付：工作流结束后终止进度轮询协程，将最终状态（成功/复用/失败/取消）就地更新回原进度消息；若无原消息则作为独立通知发出。
///
/// # 参数
/// - `app_context`: 全局应用上下文引用。
/// - `plan`: 转存规划，包含源消息链接与目标频道信息。
/// - `notify_chat_id`: 发起用户或管理通知应送达的聊天会话 ID。
/// - `progress_message_id`: 前置命令即时发送的等待卡片消息 ID，用于就地编辑展示进度与最终结果。
/// - `client_ids`: 转存涉及的 TDLib 客户端角色分配（interaction 交互客户端、worker 工作客户端等）。
/// - `admission`: 准入控制卫兵，持有期间阻止调度器在排干前强行终止任务。
pub(in crate::tgbot::transfer) fn spawn_transfer_job(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    plan: types::TransferPlan,
    notify_chat_id: i64,
    progress_message_id: Option<i64>,
    client_ids: crate::config::TransferClientIds,
    admission: crate::app_context::TransferAdmissionGuard,
) {
    tokio::spawn(async move {
        let _admission = admission;
        let source_link = plan.source_link.clone();
        let target_chat_id = plan.target_chat_id;
        tracing::info!(
            notify_chat_id,
            target_chat_id,
            progress_message_id,
            "transfer background task queued"
        );
        let progress_done = Arc::new(AtomicBool::new(false));
        let progress_handle = progress_message_id.map(|message_id| {
            let progress_plan = plan.clone();
            let progress_done = progress_done.clone();
            tracing::debug!(
                notify_chat_id,
                target_chat_id,
                progress_message_id = message_id,
                "transfer progress updater started"
            );
            let app_context = app_context.clone();
            tokio::spawn(async move {
                update_transfer_progress_message(
                    app_context,
                    progress_plan,
                    notify_chat_id,
                    message_id,
                    client_ids.interaction,
                    progress_done,
                )
                .await;
            })
        });
        let _permit = app_context.transfer_runtime.acquire_transfer_slot().await;
        tracing::info!(
            notify_chat_id,
            target_chat_id,
            "transfer background task acquired concurrency slot"
        );

        let result = workflow::transfer(app_context.clone(), plan, client_ids).await;
        let mut should_send_separate_result = progress_message_id.is_none();
        // 最终结果必须最后写入进度消息；先停止轮询任务，避免后台进度刷新覆盖“完成/失败”文本。
        progress_done.store(true, Ordering::SeqCst);
        if let Some(handle) = progress_handle {
            handle.abort();
            // 等待 abort 生效，避免进度编辑请求晚于最终结果返回后覆盖最终面板。
            let _ = handle.await;
            tracing::debug!(
                notify_chat_id,
                target_chat_id,
                "transfer progress updater stopped"
            );
        }

        if let Some(message_id) = progress_message_id
            && let Err(err) = edit_transfer_progress_for_outcome(
                &source_link,
                target_chat_id,
                &result,
                notify_chat_id,
                message_id,
                client_ids.interaction,
            )
            .await
        {
            tracing::warn!("edit final transfer progress failed: {:#}", err);
            // 用户要求整个转存生命周期只使用原进度消息；编辑失败时不再补发第二条结果卡。
            should_send_separate_result = false;
        }

        if !should_send_separate_result {
            if let Err(err) = &result {
                tracing::error!(
                    notify_chat_id,
                    target_chat_id,
                    error = %err,
                    "transfer background task finished with error"
                );
            } else {
                tracing::info!(
                    notify_chat_id,
                    target_chat_id,
                    "transfer background task finished"
                );
            }
            return;
        }

        tracing::debug!(
            notify_chat_id,
            target_chat_id,
            "sending separate transfer outcome message"
        );
        // result 会被发送函数消费；先保存错误摘要，避免为了日志克隆完整结果。
        let result_error = result.as_ref().err().map(|err| format!("{err:#}"));
        let send_result = send_transfer_outcome(
            &source_link,
            target_chat_id,
            result,
            notify_chat_id,
            client_ids.interaction,
        )
        .await;

        if let Err(err) = send_result {
            tracing::error!("send transfer outcome failed: {:#}", err);
        } else if let Some(err) = result_error {
            tracing::error!(
                notify_chat_id,
                target_chat_id,
                error = %err,
                "transfer background task finished with error"
            );
        } else {
            tracing::info!(
                notify_chat_id,
                target_chat_id,
                "transfer background task finished"
            );
        }
    });
}

/// 派发恢复任务。
///
/// 场景与行为说明：
/// - 手动恢复：管理员通过 `/job resume` 触发时可传入原任务卡片消息 ID，恢复期间持续编辑该消息，并在结束时更新为完成/失败卡片；
/// - 自动启动恢复：系统启动时自动恢复未完成的任务，此时没有可编辑的原消息，将在任务完成后以独立结果通知收尾。
///
/// # 参数
/// - `app_context`: 全局应用上下文。
/// - `job`: 待恢复的转存任务持久化模型对象。
/// - `client_ids`: TDLib 客户端角色分配 ID 组合。
/// - `progress_message_id`: 可选的交互卡片消息 ID（用于就地刷新进度与结果）。
pub(in crate::tgbot::transfer) fn spawn_recovery_job(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    job: crate::db::transfer_job::Model,
    client_ids: crate::config::TransferClientIds,
    progress_message_id: Option<i64>,
) {
    let Some(admission) = app_context.transfer_runtime.try_admit_transfer() else {
        tracing::info!(
            job_id = job.id,
            "recovery job not started while executor drains"
        );
        return;
    };
    tokio::spawn(async move {
        let _admission = admission;
        let notify_chat_id = job.request_chat_id;
        let job_id = job.id;
        let source_link = job.source_link.clone();
        let target_chat_id = job.target_chat_id;
        tracing::info!(
            job_id,
            notify_chat_id,
            target_chat_id,
            progress_message_id,
            "recovery job queued"
        );
        let progress_done = Arc::new(AtomicBool::new(false));
        let progress_handle = progress_message_id.map(|message_id| {
            let app_context = app_context.clone();
            let source_link = source_link.clone();
            let progress_done = progress_done.clone();
            tracing::debug!(
                job_id,
                notify_chat_id,
                target_chat_id,
                progress_message_id = message_id,
                "recovery progress updater started"
            );
            tokio::spawn(async move {
                update_recovery_progress_message(
                    app_context,
                    RecoveryProgressUpdate {
                        job_id,
                        source_link,
                        target_chat_id,
                        notify_chat_id,
                        message_id,
                        client_id: client_ids.interaction,
                        done: progress_done,
                    },
                )
                .await;
            })
        });
        let _permit = app_context.transfer_runtime.acquire_transfer_slot().await;
        tracing::info!(
            job_id,
            notify_chat_id,
            target_chat_id,
            "recovery job acquired concurrency slot"
        );

        let result = workflow::resume_one_job(app_context.clone(), job, client_ids).await;
        progress_done.store(true, Ordering::SeqCst);
        if let Some(handle) = progress_handle {
            handle.abort();
            let _ = handle.await;
            tracing::debug!(
                job_id,
                notify_chat_id,
                target_chat_id,
                "recovery progress updater stopped"
            );
        }

        if let Some(message_id) = progress_message_id {
            if let Err(err) = edit_transfer_progress_for_outcome(
                &source_link,
                target_chat_id,
                &result,
                notify_chat_id,
                message_id,
                client_ids.interaction,
            )
            .await
            {
                tracing::warn!(job_id, error = %err, "edit final recovery progress failed");
            }
            if let Err(err) = &result {
                tracing::error!(
                    job_id,
                    notify_chat_id,
                    target_chat_id,
                    error = %err,
                    "recovery job finished with error"
                );
            } else {
                tracing::info!(
                    job_id,
                    notify_chat_id,
                    target_chat_id,
                    "recovery job finished"
                );
            }
            return;
        }

        // result 会被发送函数消费；先保存错误摘要，避免为了日志克隆完整结果。
        let result_error = result.as_ref().err().map(|err| format!("{err:#}"));
        let send_result = send_recovery_outcome(
            job_id,
            &source_link,
            target_chat_id,
            result,
            notify_chat_id,
            client_ids.interaction,
        )
        .await;

        if let Err(err) = send_result {
            tracing::error!("send recovery outcome failed: {:#}", err);
        } else if let Some(err) = result_error {
            tracing::error!(
                job_id,
                notify_chat_id,
                target_chat_id,
                error = %err,
                "recovery job finished with error"
            );
        } else {
            tracing::info!(
                job_id,
                notify_chat_id,
                target_chat_id,
                "recovery job finished"
            );
        }
    });
}
