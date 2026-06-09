// 转存启动阶段：
// - source_link + target_chat_id 是业务查重维度，用来复用成功任务和阻止重复活跃任务。
// - request_chat_id + request_message_id 是请求幂等维度，只兜底处理 TDLib/网络重复投递同一条命令。
// - 两层语义不能混用：前者决定“是不是同一个转存”，后者决定“这条命令是否已经处理过”。

use crate::db;

use super::super::types::{SourceKind, TransferPlan};
use super::super::{spider, store};
use super::TransferOutcome;
use super::control::apply_job_control;
use super::guard::{JobGuard, acquire_job_guard, acquire_source_target_create_guard};
use super::result_link::refresh_stored_result_link;

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
    client_ids: crate::config::TransferClientIds,
) -> anyhow::Result<TransferStart> {
    tracing::debug!(
        request_chat_id = plan.request_chat_id,
        request_message_id = plan.request_message_id,
        target_chat_id = plan.target_chat_id,
        "transfer start resolving"
    );
    let _guard =
        acquire_source_target_create_guard(plan.source_link.clone(), plan.target_chat_id).await;
    tracing::debug!(
        request_chat_id = plan.request_chat_id,
        request_message_id = plan.request_message_id,
        target_chat_id = plan.target_chat_id,
        "transfer source-target create guard acquired"
    );

    // 业务查重第一层：
    // 同一个源链接转到同一个目标 chat，如果已经成功完成，直接返回历史结果。
    // 这里不看 request_message_id，因为不同命令重复转存同一链接时也应复用成功结果。
    if let Some(old) =
        store::find_success_job_by_source_target(&plan.source_link, plan.target_chat_id).await?
    {
        let link = refresh_stored_result_link(
            old.id,
            old.target_chat_id,
            old.result_message_id,
            &old.result_message_link,
            client_ids.upload,
        )
        .await?;
        tracing::info!(
            job_id = old.id,
            target_chat_id = plan.target_chat_id,
            "reuse successful transfer result"
        );
        return Ok(TransferStart::Outcome(TransferOutcome::Reused {
            job_id: old.id,
            link,
        }));
    }

    // 业务查重第二层：
    // 同一个源链接转到同一个目标 chat，如果已有活跃任务，不能再创建新任务。
    // 这一步能处理“用户发送两条相同命令但 message_id 不同”的情况。
    if let Some(old) =
        store::find_active_job_by_source_target(&plan.source_link, plan.target_chat_id).await?
    {
        tracing::info!(
            job_id = old.id,
            status = %old.status,
            target_chat_id = plan.target_chat_id,
            "matched active transfer job"
        );
        return Ok(active_job_start(old, &plan));
    }

    // 请求级幂等兜底：
    // TDLib/网络波动可能导致同一条命令 update 被重复投递，此时 request_chat_id
    // 和 request_message_id 完全相同。即使上面的 source-target 查不到 active/success
    // （例如第一次处理已失败/取消），也不能让同一条命令再次创建新 job。
    // 这一步不是业务查重；业务查重只由 source_link + target_chat_id 决定。
    if let Some(old) =
        store::find_job_by_request(plan.request_chat_id, plan.request_message_id).await?
    {
        tracing::info!(
            job_id = old.id,
            status = %old.status,
            request_chat_id = plan.request_chat_id,
            request_message_id = plan.request_message_id,
            "matched idempotent transfer request"
        );
        return request_job_start(old, client_ids.upload).await;
    }

    create_new_job_start(plan, client_ids).await
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
        // 不同请求命中同一个 source-target 活跃任务时，只提示“已有任务在跑”，不重复派发执行器。
        // 同一请求命中 active 任务则可能是重复 update，需要走 Resume 兜底恢复。
        TransferStart::Outcome(TransferOutcome::Running { job_id: old.id })
    } else {
        TransferStart::Resume(old)
    }
}

/// 将同一请求已存在的任务转换为下一步动作。
async fn request_job_start(
    old: db::transfer_job::Model,
    upload_client_id: i32,
) -> anyhow::Result<TransferStart> {
    // 同一条命令重复投递时按已有任务状态返回确定结果：
    // - pending/running：恢复执行，避免上次后台任务已丢失。
    // - paused/cancelling/cancelled：返回状态，不重新创建。
    // - success 且有结果链接：返回结果。
    // - failed/partial 且无结果链接：报错，不让同一条命令自动重试成新 job。
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
    } else if let Some(link) = old.result_message_link.as_deref() {
        let link = refresh_stored_result_link(
            old.id,
            old.target_chat_id,
            old.result_message_id,
            link,
            upload_client_id,
        )
        .await?;
        Ok(TransferStart::Outcome(TransferOutcome::Reused {
            job_id: old.id,
            link,
        }))
    } else {
        anyhow::bail!("duplicated request without reusable result link");
    }
}

/// 抓取源消息并创建新的转存任务。
async fn create_new_job_start(
    plan: TransferPlan,
    client_ids: crate::config::TransferClientIds,
) -> anyhow::Result<TransferStart> {
    // 抓取源消息（单条或相册）。
    tracing::info!(
        request_chat_id = plan.request_chat_id,
        request_message_id = plan.request_message_id,
        target_chat_id = plan.target_chat_id,
        "spider source messages started"
    );
    let bundle = match plan.source_kind {
        SourceKind::Link => {
            if plan.preferred_source_client_role == crate::config::ClientRole::Bot {
                let bot_client_id = client_ids.get(crate::config::ClientRole::Bot)?;
                let user_client_id = client_ids.get(crate::config::ClientRole::User)?;
                spider::spider_link_bot_first(
                    plan.source_link.clone(),
                    bot_client_id,
                    user_client_id,
                )
                .await?
            } else {
                let client_id = client_ids.get(plan.preferred_source_client_role)?;
                spider::spider_message(
                    plan.source_link.clone(),
                    client_id,
                    plan.preferred_source_client_role,
                )
                .await?
            }
        }
        SourceKind::BotMessage => {
            let bot_client_id = client_ids.get(crate::config::ClientRole::Bot)?;
            let source_chat_id = plan
                .source_message_chat_id
                .ok_or_else(|| anyhow::anyhow!("bot message source chat_id missing"))?;
            let source_message_id = plan
                .source_message_id
                .ok_or_else(|| anyhow::anyhow!("bot message source message_id missing"))?;
            spider::spider_bot_visible_message(source_chat_id, source_message_id, bot_client_id)
                .await?
        }
    };
    tracing::info!(
        request_chat_id = plan.request_chat_id,
        request_message_id = plan.request_message_id,
        source_chat_id = bundle.source_chat_id,
        source_message_id = bundle.source_message_id,
        source_album_id = bundle.source_album_id,
        message_count = bundle.messages.len(),
        "spider source messages completed"
    );

    // 创建主任务并对齐子项；创建完成后释放 source-target 锁，实际执行由 job_id 锁保护。
    let job = store::create_job(&plan, &bundle).await?;
    tracing::info!(
        job_id = job.id,
        request_chat_id = plan.request_chat_id,
        request_message_id = plan.request_message_id,
        source_chat_id = bundle.source_chat_id,
        source_message_id = bundle.source_message_id,
        source_album_id = bundle.source_album_id,
        target_chat_id = plan.target_chat_id,
        total_items = bundle.messages.len(),
        "created transfer job"
    );
    // 新任务从创建子项前就持有 job 锁，避免 `/j stop` 在子项写入前误判“无执行器”。
    match acquire_job_guard(job.id).await {
        Some(job_guard) => {
            if let Some(outcome) = apply_job_control(job.id).await? {
                tracing::info!(
                    job_id = job.id,
                    "transfer job control applied before item creation"
                );
                Ok(TransferStart::Outcome(outcome))
            } else {
                let _ = store::ensure_items_for_bundle(job.id, &bundle).await?;
                tracing::debug!(
                    job_id = job.id,
                    total_items = bundle.messages.len(),
                    "transfer job items ensured"
                );
                Ok(TransferStart::Run(job, bundle.messages, job_guard))
            }
        }
        None => {
            tracing::info!(
                job_id = job.id,
                "transfer job guard already held after creation"
            );
            Ok(TransferStart::Outcome(TransferOutcome::Running {
                job_id: job.id,
            }))
        }
    }
}
