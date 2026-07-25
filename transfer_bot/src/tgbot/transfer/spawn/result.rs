// 后台任务执行结果通知。
// `spawn.rs` 只负责任务生命周期，这里集中处理 `TransferOutcome` 到回复卡片的映射。

use super::super::outcome::{
    send_cancelled_message, send_cancelling_message, send_failure_message,
    send_history_hit_message, send_paused_message, send_running_message,
};
use super::super::workflow;

/// 发送新转存任务的最终结果。
pub(super) async fn send_transfer_outcome(
    source_link: &str,
    target_chat_id: i64,
    result: anyhow::Result<workflow::TransferOutcome>,
    notify_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    match result {
        Ok(workflow::TransferOutcome::Reused { job_id, link }) => {
            send_history_hit_message(
                "已存在历史转存结果",
                source_link,
                target_chat_id,
                job_id,
                &link,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Ok(workflow::TransferOutcome::Running { job_id }) => {
            send_running_message(
                "相同链接正在转存中",
                source_link,
                target_chat_id,
                job_id,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Ok(workflow::TransferOutcome::Paused { job_id }) => {
            send_paused_message(
                "相同链接任务已暂停",
                source_link,
                target_chat_id,
                job_id,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Ok(workflow::TransferOutcome::Cancelling { job_id }) => {
            send_cancelling_message(
                "相同链接任务正在停止",
                source_link,
                target_chat_id,
                job_id,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Ok(workflow::TransferOutcome::Cancelled { job_id }) => {
            send_cancelled_message(
                "转存任务已停止",
                source_link,
                target_chat_id,
                job_id,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Ok(workflow::TransferOutcome::Completed { job_id, link }) => {
            send_history_hit_message(
                "转存完成",
                source_link,
                target_chat_id,
                job_id,
                &link,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Err(err) => {
            send_failure_message(
                "转存失败",
                source_link,
                target_chat_id,
                None,
                err,
                notify_chat_id,
                client_id,
            )
            .await
        }
    }
}

/// 发送启动恢复任务的最终结果。
pub(super) async fn send_recovery_outcome(
    recovery_job_id: i64,
    source_link: &str,
    target_chat_id: i64,
    result: anyhow::Result<workflow::TransferOutcome>,
    notify_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    match result {
        Ok(workflow::TransferOutcome::Reused { job_id, link }) => {
            send_history_hit_message(
                "恢复任务命中历史结果",
                source_link,
                target_chat_id,
                job_id,
                &link,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Ok(workflow::TransferOutcome::Running { job_id }) => {
            send_running_message(
                "恢复任务继续执行中",
                source_link,
                target_chat_id,
                job_id,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Ok(workflow::TransferOutcome::Paused { job_id }) => {
            send_paused_message(
                "恢复任务处于暂停状态",
                source_link,
                target_chat_id,
                job_id,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Ok(workflow::TransferOutcome::Cancelling { job_id }) => {
            send_cancelling_message(
                "恢复任务正在停止",
                source_link,
                target_chat_id,
                job_id,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Ok(workflow::TransferOutcome::Cancelled { job_id }) => {
            send_cancelled_message(
                "恢复任务已停止",
                source_link,
                target_chat_id,
                job_id,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Ok(workflow::TransferOutcome::Completed { job_id, link }) => {
            send_history_hit_message(
                "恢复任务完成",
                source_link,
                target_chat_id,
                job_id,
                &link,
                notify_chat_id,
                client_id,
            )
            .await
        }
        Err(err) => {
            send_failure_message(
                &format!("恢复任务失败，job_id={recovery_job_id}"),
                source_link,
                target_chat_id,
                Some(recovery_job_id),
                err,
                notify_chat_id,
                client_id,
            )
            .await
        }
    }
}
