// 任务控制状态处理：
// - 暂停时停止当前工作流
// - 停止时执行取消收尾并释放文件引用

use crate::tgbot::transfer::store;

use super::{TransferOutcome, file_delete_delay_minutes};

/// 检查用户控制状态，并在需要时执行暂停/取消收尾。
///
/// 返回 Some 表示当前任务应停止继续执行；返回 None 表示可以继续。
pub(super) async fn apply_job_control(job_id: i64) -> anyhow::Result<Option<TransferOutcome>> {
    let Some(status) = store::get_job_status(job_id).await? else {
        anyhow::bail!("job not found: {}", job_id);
    };

    match status.as_str() {
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING => Ok(None),
        store::JOB_STATUS_PAUSED => Ok(Some(TransferOutcome::Paused { job_id })),
        store::JOB_STATUS_CANCELLING
        | store::JOB_STATUS_CANCEL_FINALIZING
        | store::JOB_STATUS_CANCELLED => {
            store::cancel_job_now(job_id, "cancelled by user", file_delete_delay_minutes()).await?;
            Ok(Some(TransferOutcome::Cancelled { job_id }))
        }
        status if store::is_finished_job_status(status) => {
            anyhow::bail!("job already finished during workflow: {}", status)
        }
        other => anyhow::bail!("unknown job status during workflow: {}", other),
    }
}

/// finish_job 被用户控制状态抢先占用时，统一切回控制流程。
pub(super) async fn finish_skipped_by_control(job_id: i64) -> anyhow::Result<TransferOutcome> {
    if let Some(outcome) = apply_job_control(job_id).await? {
        return Ok(outcome);
    }
    anyhow::bail!(
        "job finish skipped but no control outcome, job_id={}",
        job_id
    )
}
