// 任务控制状态处理：
// - 暂停时停止当前工作流
// - 停止时执行取消收尾并释放文件引用

use crate::tgbot::transfer::store;

use super::{TransferOutcome, file_delete_delay_minutes};

/// 检查用户控制状态，并在需要时执行暂停或取消收尾。
///
/// 在流水线的各循环轮次与关键检查点（准备前、下载后、上传前）调用：
/// - 若状态为 `pending` 或 `running`，返回 `None` 表示可安全继续执行；
/// - 若检测到被置为 `paused`，返回 `Some(TransferOutcome::Paused)` 退出执行；
/// - 若检测到处于 `cancelling` 等停止态，调用 `cancel_job_now` 完成收尾并释放物理文件引用，返回 `Some(TransferOutcome::Cancelled)`。
///
/// # 参数
/// - `app_context`: 全局应用上下文。
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - `Some(TransferOutcome)` 表示当前任务已被控制态中断，调用方应立即终止；`None` 表示可以继续。
pub(super) async fn apply_job_control(
    app_context: &crate::app_context::AppContext,
    job_id: i64,
) -> anyhow::Result<Option<TransferOutcome>> {
    let Some(status) = store::get_job_status(job_id).await? else {
        anyhow::bail!("job not found: {job_id}");
    };

    match status.as_str() {
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING => Ok(None),
        store::JOB_STATUS_PAUSED => Ok(Some(TransferOutcome::Paused { job_id })),
        store::JOB_STATUS_CANCELLING
        | store::JOB_STATUS_CANCEL_FINALIZING
        | store::JOB_STATUS_CANCELLED => {
            store::cancel_job_now(
                job_id,
                "cancelled by user",
                file_delete_delay_minutes(app_context),
            )
            .await?;
            Ok(Some(TransferOutcome::Cancelled { job_id }))
        }
        status if store::is_finished_job_status(status) => {
            anyhow::bail!("job already finished during workflow: {status}")
        }
        other => anyhow::bail!("unknown job status during workflow: {other}"),
    }
}

/// 当 `finish_job` 被用户并发控制状态（暂停或取消）抢先占用时，统一切回控制流程产物。
///
/// # 参数
/// - `app_context`: 全局应用上下文。
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - 对应的中断产物（如 `TransferOutcome::Paused` 或 `TransferOutcome::Cancelled`）。
pub(super) async fn finish_skipped_by_control(
    app_context: &crate::app_context::AppContext,
    job_id: i64,
) -> anyhow::Result<TransferOutcome> {
    if let Some(outcome) = apply_job_control(app_context, job_id).await? {
        return Ok(outcome);
    }
    anyhow::bail!("job finish skipped but no control outcome, job_id={job_id}")
}
