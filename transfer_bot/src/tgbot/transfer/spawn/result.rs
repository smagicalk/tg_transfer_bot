// 后台任务执行结果通知。
// `spawn.rs` 只负责任务生命周期，这里集中处理 `TransferOutcome` 到回复卡片的映射。

use super::super::outcome::{
    send_cancelled_message, send_cancelling_message, send_failure_message,
    send_history_hit_message, send_paused_message, send_running_message,
};
use super::super::workflow;

/// 发送新转存任务的最终结果。
///
/// 针对 `workflow::TransferOutcome` 各状态进行映射并发送对应的卡片消息：
/// - `Reused`: 发现完全相同的来源和文件哈希已存在，直接复用历史转存结果并展示跳转按钮；
/// - `Running`: 相同的链接已经在另一个并发任务中运行；
/// - `Paused`: 该链接对应的历史任务处于已暂停状态；
/// - `Cancelling`: 该任务已被请求取消且正在停止过程中；
/// - `Cancelled`: 任务已经终止取消；
/// - `Completed`: 本次全新转存执行完成，展示目标新消息链接与定位；
/// - `Err`: 转存中途发生错误，生成智能诊断与重试引导卡片。
///
/// # 参数
/// - `source_link`: 来源 Telegram 链接。
/// - `target_chat_id`: 目标频道 ID。
/// - `result`: 工作流返回的 `TransferOutcome` 或错误。
/// - `notify_chat_id`: 接收结果通知的聊天 ID。
/// - `client_id`: 负责发送通知的交互客户端 ID。
///
/// # 返回值
/// - `Ok(())` 表示消息发送成功；`Err` 表示 TDLib 发送失败。
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
///
/// 针对中断后被恢复的 `recovery_job_id`，将结果通知发送给发起者或管理员。
///
/// # 参数
/// - `recovery_job_id`: 正在恢复的历史任务数据库主键 ID。
/// - `source_link`: 来源 Telegram 链接。
/// - `target_chat_id`: 目标频道 ID。
/// - `result`: 恢复工作流的执行产物。
/// - `notify_chat_id`: 通知目标聊天 ID。
/// - `client_id`: 发送 TDLib 客户端 ID。
///
/// # 返回值
/// - `Ok(())` 表示通知发送成功。
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
