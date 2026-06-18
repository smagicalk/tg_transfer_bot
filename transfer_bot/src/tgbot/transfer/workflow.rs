// 转存任务执行流程模块：
// - `start`：判断复用、恢复或创建新任务
// - `runner`：持有 job 锁后的下载、准备、上传主流程
// - `recovery`：启动恢复和单任务恢复
// - `gc`：文件删除队列消费

use super::types::TransferPlan;

mod control;
mod gc;
mod guard;
mod recovery;
mod result_link;
mod runner;
mod start;
#[cfg(test)]
mod tests;
mod upload;

pub(super) use gc::run_file_gc_loop;
pub(super) use guard::is_job_running_in_process;
pub(super) use recovery::{
    maybe_send_startup_setup_guide, recover_unfinished_jobs, resume_one_job,
};
pub(in crate::tgbot::transfer) use result_link::{
    refresh_stored_result_link, refresh_stored_result_messages,
};
use runner::run_job_inner;
use start::{TransferStart, build_transfer_start};

/// 转存命令的执行结果。
#[derive(Debug, Clone)]
pub(super) enum TransferOutcome {
    /// 复用历史成功任务，直接返回已有链接。
    Reused { job_id: i64, link: String },
    /// 命中相同 source_link + target_chat_id 的进行中任务。
    Running { job_id: i64 },
    /// 任务已被用户暂停，等待手动恢复。
    Paused { job_id: i64 },
    /// 任务已经进入停止流程。
    Cancelling { job_id: i64 },
    /// 任务已被用户停止。
    Cancelled { job_id: i64 },
    /// 本次执行成功完成，并返回新生成链接。
    Completed { job_id: i64, link: String },
}

/// 执行单次转存任务（命令入口）。
pub(super) async fn transfer(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    plan: TransferPlan,
    client_ids: crate::config::TransferClientIds,
) -> anyhow::Result<TransferOutcome> {
    let start = build_transfer_start(plan, client_ids).await?;
    match start {
        TransferStart::Outcome(outcome) => Ok(outcome),
        TransferStart::Resume(job) => resume_one_job(app_context.clone(), job, client_ids).await,
        TransferStart::Run(job, messages, _guard) => {
            run_job_inner(app_context, job, messages, client_ids).await
        }
    }
}

/// 文件删除延迟（分钟）：
/// 从 config.json 读取 `transfer_config.file_delete_delay_minutes`。
fn file_delete_delay_minutes() -> i64 {
    super::runtime_config().file_delete_delay_minutes.max(0)
}
