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
pub(super) use recovery::{recover_unfinished_jobs, resume_one_job};
pub(in crate::tgbot::transfer) use result_link::{
    extract_tdlib_message_id_from_stored_link, refresh_stored_result_link,
    refresh_stored_result_messages,
};
use runner::run_job_inner;
use start::{TransferStart, build_transfer_start};

/// 转存命令工作流的执行最终结果。
#[derive(Debug, Clone)]
pub(super) enum TransferOutcome {
    /// 复用历史成功任务，直接返回已有的目标消息跳转链接。
    Reused {
        /// 被复用的历史任务 ID。
        job_id: i64,
        /// 目标消息有效超链接。
        link: String,
    },
    /// 发现相同来源与相同目标已有任务正在并发执行中。
    Running {
        /// 正在执行的任务 ID。
        job_id: i64,
    },
    /// 任务处于手动暂停状态，提示用户如何恢复。
    Paused {
        /// 处于暂停状态的任务 ID。
        job_id: i64,
    },
    /// 任务正在协作式取消收尾流程中。
    Cancelling {
        /// 正在取消的任务 ID。
        job_id: i64,
    },
    /// 任务已被用户停止确认。
    Cancelled {
        /// 已取消的任务 ID。
        job_id: i64,
    },
    /// 本次全新的转存流水线圆满完成，生成了全新的目标消息与链接。
    Completed {
        /// 新建的任务 ID。
        job_id: i64,
        /// 目标新消息的访问链接。
        link: String,
    },
}

/// 执行单次转存任务（转存业务工作流的核心调度总入口）。
///
/// 逻辑分支：
/// 1. `build_transfer_start`: 爬取源消息并结合数据库判断启动方式；
/// 2. 若命中直接产物（如历史复用、运行冲突等），直接返回 `TransferOutcome`；
/// 3. 若检测到存在可接续历史中断任务，转入 `resume_one_job` 恢复执行；
/// 4. 若为全新任务，获取进程内任务排他守卫后转入 `run_job_inner` 展开下载、缓存与发送。
///
/// # 参数
/// - `app_context`: 全局应用上下文。
/// - `plan`: 转存规划（包含来源链接、目标频道、调用权限等）。
/// - `client_ids`: TDLib 客户端角色分配配置。
///
/// # 返回值
/// - 执行完成后的最终结果产物 `TransferOutcome`。
pub(super) async fn transfer(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    plan: TransferPlan,
    client_ids: crate::config::TransferClientIds,
) -> anyhow::Result<TransferOutcome> {
    let start = build_transfer_start(app_context.clone(), plan, client_ids).await?;
    match start {
        TransferStart::Outcome(outcome) => Ok(outcome),
        TransferStart::Resume(job) => resume_one_job(app_context.clone(), job, client_ids).await,
        TransferStart::Run(job, messages, _guard) => {
            run_job_inner(app_context, job, messages, client_ids).await
        }
    }
}

/// 从运行时配置中读取文件缓存删除安全延迟时间（分钟）。
///
/// # 参数
/// - `app_context`: 全局应用上下文。
///
/// # 返回值
/// - 延迟分钟数（非负整数）。
fn file_delete_delay_minutes(app_context: &crate::app_context::AppContext) -> i64 {
    app_context
        .transfer_runtime
        .runtime_config()
        .file_delete_delay_minutes
        .max(0)
}
