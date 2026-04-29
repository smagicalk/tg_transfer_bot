// 转存启动阶段：
// - 优先复用 source_link + target_chat_id 的历史成功结果
// - 命中活跃任务时返回当前状态
// - 否则抓取源消息、创建任务和子项

use crate::db;

use super::super::types::TransferPlan;
use super::super::{spider, store};
use super::TransferOutcome;
use super::control::apply_job_control;
use super::guard::{JobGuard, acquire_job_guard, acquire_source_target_create_guard};

/// 转存入口完成创建阶段后的下一步动作。
pub(super) enum TransferStart {
    /// 已经可以直接返回结果，不需要执行后台流程。
    Outcome(TransferOutcome),
    /// 命中同一请求的未完成任务，释放创建锁后再恢复。
    Resume(db::transfer_job::Model),
    /// 新建任务完成，释放创建锁后执行下载与上传。
    Run(
        db::transfer_job::Model,
        Vec<tdlib_rs::types::Message>,
        JobGuard,
    ),
}

/// 判断本次 `/transfer` 应复用、恢复还是创建新任务。
pub(super) async fn build_transfer_start(
    plan: TransferPlan,
    client_id: i32,
) -> anyhow::Result<TransferStart> {
    let _guard =
        acquire_source_target_create_guard(plan.source_link.clone(), plan.target_chat_id).await;

    // 新去重语义：按 source_link + target_chat_id 判断是否已转存完成。
    if let Some(old) =
        store::find_success_job_by_source_target(&plan.source_link, plan.target_chat_id).await?
        && let Some(link) = old.result_message_link
    {
        return Ok(TransferStart::Outcome(TransferOutcome::Reused { link }));
    }

    if let Some(old) =
        store::find_active_job_by_source_target(&plan.source_link, plan.target_chat_id).await?
    {
        return Ok(active_job_start(old, &plan));
    }

    if let Some(old) =
        store::find_job_by_request(plan.request_chat_id, plan.request_message_id).await?
    {
        return request_job_start(old).await;
    }

    create_new_job_start(plan, client_id).await
}

/// 将已存在的活跃任务转换为本次命令结果。
fn active_job_start(old: db::transfer_job::Model, plan: &TransferPlan) -> TransferStart {
    if old.status == store::JOB_STATUS_PAUSED {
        TransferStart::Outcome(TransferOutcome::Paused { job_id: old.id })
    } else if matches!(
        old.status.as_str(),
        store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING
    ) {
        TransferStart::Outcome(TransferOutcome::Cancelling { job_id: old.id })
    } else if old.request_chat_id != plan.request_chat_id
        || old.request_message_id != plan.request_message_id
    {
        // 只有同一请求消息命中的 active 任务才继续走恢复，其他请求直接提示进行中。
        TransferStart::Outcome(TransferOutcome::Running { job_id: old.id })
    } else {
        TransferStart::Resume(old)
    }
}

/// 将同一请求已存在的任务转换为下一步动作。
async fn request_job_start(old: db::transfer_job::Model) -> anyhow::Result<TransferStart> {
    // 请求幂等：
    // - 已完成任务（success/failed/partial）直接跳过
    // - 未完成任务（pending/running）走恢复执行
    if matches!(
        old.status.as_str(),
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
    ) {
        Ok(TransferStart::Resume(old))
    } else if old.status == store::JOB_STATUS_PAUSED {
        Ok(TransferStart::Outcome(TransferOutcome::Paused {
            job_id: old.id,
        }))
    } else if matches!(
        old.status.as_str(),
        store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING
    ) {
        Ok(TransferStart::Outcome(TransferOutcome::Cancelling {
            job_id: old.id,
        }))
    } else if old.status == store::JOB_STATUS_CANCELLED {
        Ok(TransferStart::Outcome(TransferOutcome::Cancelled {
            job_id: old.id,
        }))
    } else if let Some(link) = old.result_message_link {
        Ok(TransferStart::Outcome(TransferOutcome::Reused { link }))
    } else {
        anyhow::bail!("duplicated request without reusable result link");
    }
}

/// 抓取源消息并创建新的转存任务。
async fn create_new_job_start(plan: TransferPlan, client_id: i32) -> anyhow::Result<TransferStart> {
    // 抓取源消息（单条或相册）。
    let bundle = spider::spider_message(plan.source_link.clone(), client_id).await?;

    // 创建主任务并对齐子项；创建完成后释放 source-target 锁，实际执行由 job_id 锁保护。
    let job = store::create_job(&plan, &bundle).await?;
    // 新任务从创建子项前就持有 job 锁，避免 `/j stop` 在子项写入前误判“无执行器”。
    match acquire_job_guard(job.id).await {
        Some(job_guard) => {
            if let Some(outcome) = apply_job_control(job.id).await? {
                Ok(TransferStart::Outcome(outcome))
            } else {
                let _ = store::ensure_items_for_bundle(job.id, &bundle.messages).await?;
                Ok(TransferStart::Run(job, bundle.messages, job_guard))
            }
        }
        None => Ok(TransferStart::Outcome(TransferOutcome::Running {
            job_id: job.id,
        })),
    }
}
