// 转存恢复流程。
// 启动时恢复数据库中的 pending/running 任务，并收敛重启前已经 cancelling 的任务。

use crate::db;

use super::super::{spider, store};
use super::TransferOutcome;
use super::control::{apply_job_control, finish_skipped_by_control};
use super::guard::acquire_job_guard;
use super::runner::run_job_inner;

/// 启动时恢复数据库里未完成任务。
pub(in crate::tgbot::transfer) async fn recover_unfinished_jobs(
    client_id: i32,
) -> anyhow::Result<()> {
    // 上次退出前已经请求停止的任务，启动时先收敛为 cancelled 并释放引用。
    for job in store::list_cancelling_jobs().await? {
        tracing::info!("finalize cancelling transfer job, job_id={}", job.id);
        store::cancel_job_now(
            job.id,
            "cancelled by user before restart",
            super::file_delete_delay_hours(),
        )
        .await?;
    }

    let jobs = store::list_recoverable_jobs().await?;
    if jobs.is_empty() {
        tracing::info!("no recoverable transfer jobs");
        return Ok(());
    }

    tracing::info!("scheduling {} unfinished transfer jobs", jobs.len());
    for job in jobs {
        tracing::info!("schedule recover job, job_id={}", job.id);
        super::super::spawn_recovery_job(job, client_id);
    }
    Ok(())
}

/// 恢复单个任务：
/// - 重新抓取 source_link
/// - 对齐子项并执行
pub(in crate::tgbot::transfer) async fn resume_one_job(
    job: db::transfer_job::Model,
    client_id: i32,
) -> anyhow::Result<TransferOutcome> {
    // 恢复流程从抓取源消息开始就占用 job 运行锁，避免 stop 命令误判“无执行器”后直接释放引用。
    let _guard = match acquire_job_guard(job.id).await {
        Some(g) => g,
        None => {
            tracing::info!("job already running in this process, job_id={}", job.id);
            return Ok(TransferOutcome::Running { job_id: job.id });
        }
    };

    if let Some(outcome) = apply_job_control(job.id).await? {
        return Ok(outcome);
    }

    let bundle = spider::spider_message(job.source_link.clone(), client_id).await?;
    if let Some(outcome) = apply_job_control(job.id).await? {
        return Ok(outcome);
    }

    if !store::mark_job_running(job.id).await? {
        return finish_skipped_by_control(job.id).await;
    }
    let _ = store::ensure_items_for_bundle(job.id, &bundle.messages).await?;
    run_job_inner(job, bundle.messages, client_id).await
}
