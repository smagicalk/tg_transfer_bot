// 任务暂停、恢复、停止和启动恢复扫描相关测试。

use super::super::*;
use super::fixtures::*;
use crate::db;
use sea_orm::EntityTrait;

/// 暂停、唤醒、停止请求应按预期流转主任务状态。
#[tokio::test]
async fn test_job_control_status_flow() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let job = insert_job(JOB_STATUS_RUNNING).await?;

    let paused =
        pause_job_with_owner_scope(job.id, job.request_chat_id, Some(job.owner_user_id)).await?;
    assert_eq!(paused.status, JOB_STATUS_PAUSED);

    let resumed =
        wake_job_with_owner_scope(job.id, job.request_chat_id, Some(job.owner_user_id)).await?;
    assert_eq!(resumed.status, JOB_STATUS_PENDING);

    let cancelling =
        request_cancel_job_with_owner_scope(job.id, job.request_chat_id, Some(job.owner_user_id))
            .await?;
    assert_eq!(cancelling.status, JOB_STATUS_CANCELLING);
    Ok(())
}

/// 普通用户只能控制 owner_user_id 等于自己的任务；admin owner_scope=None 可控制全局任务。
#[tokio::test]
async fn test_job_control_respects_owner_scope() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let job = insert_job(JOB_STATUS_RUNNING).await?;

    assert!(
        pause_job_with_owner_scope(job.id, job.request_chat_id, Some(job.owner_user_id + 1))
            .await
            .is_err()
    );

    let paused = pause_job_with_owner_scope(job.id, job.request_chat_id, None).await?;
    assert_eq!(paused.status, JOB_STATUS_PAUSED);
    assert_eq!(paused.owner_user_id, job.owner_user_id);
    Ok(())
}

/// pending/running 任务允许手动唤醒，用于后台执行器丢失后的补派发。
#[tokio::test]
async fn test_wake_job_accepts_pending_and_running() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let pending = insert_job(JOB_STATUS_PENDING).await?;
    let running = insert_job(JOB_STATUS_RUNNING).await?;

    let woke_pending = wake_job_with_owner_scope(
        pending.id,
        pending.request_chat_id,
        Some(pending.owner_user_id),
    )
    .await?;
    assert_eq!(woke_pending.status, JOB_STATUS_PENDING);

    let woke_running = wake_job_with_owner_scope(
        running.id,
        running.request_chat_id,
        Some(running.owner_user_id),
    )
    .await?;
    assert_eq!(woke_running.status, JOB_STATUS_RUNNING);
    Ok(())
}

/// 已经完成的任务不能再被暂停、唤醒或停止。
#[tokio::test]
async fn test_finished_job_rejects_manual_control() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let job = insert_job(JOB_STATUS_SUCCESS).await?;

    assert!(
        pause_job_with_owner_scope(job.id, job.request_chat_id, Some(job.owner_user_id))
            .await
            .is_err()
    );
    assert!(
        wake_job_with_owner_scope(job.id, job.request_chat_id, Some(job.owner_user_id))
            .await
            .is_err()
    );
    assert!(
        request_cancel_job_with_owner_scope(job.id, job.request_chat_id, Some(job.owner_user_id))
            .await
            .is_err()
    );
    Ok(())
}

/// 恢复流程标记 running 时不能覆盖用户已经暂停的任务。
#[tokio::test]
async fn test_mark_job_running_does_not_overwrite_paused() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_PAUSED).await?;

    let marked = mark_job_running(job.id).await?;
    assert!(!marked);

    let job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(job.status, JOB_STATUS_PAUSED);
    Ok(())
}

/// 恢复流程标记 running 时也不能覆盖用户已经发出的停止请求。
#[tokio::test]
async fn test_mark_job_running_reports_cancelling_conflict() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_CANCELLING).await?;

    let marked = mark_job_running(job.id).await?;
    assert!(!marked);

    let job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(job.status, JOB_STATUS_CANCELLING);
    Ok(())
}

/// 启动恢复只恢复 pending/running；cancelling 需要单独扫描收尾。
#[tokio::test]
async fn test_recoverable_and_cancelling_job_scan() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let pending = insert_job(JOB_STATUS_PENDING).await?;
    let running = insert_job(JOB_STATUS_RUNNING).await?;
    let paused = insert_job(JOB_STATUS_PAUSED).await?;
    let cancelling = insert_job(JOB_STATUS_CANCELLING).await?;
    let finalizing = insert_job(JOB_STATUS_CANCEL_FINALIZING).await?;
    let cancelled = insert_job(JOB_STATUS_CANCELLED).await?;

    let recoverable_ids = list_recoverable_jobs()
        .await?
        .into_iter()
        .map(|job| job.id)
        .collect::<std::collections::HashSet<_>>();
    assert!(recoverable_ids.contains(&pending.id));
    assert!(recoverable_ids.contains(&running.id));
    assert!(!recoverable_ids.contains(&paused.id));
    assert!(!recoverable_ids.contains(&cancelling.id));
    assert!(!recoverable_ids.contains(&finalizing.id));
    assert!(!recoverable_ids.contains(&cancelled.id));

    let cancelling_ids = list_cancelling_jobs()
        .await?
        .into_iter()
        .map(|job| job.id)
        .collect::<std::collections::HashSet<_>>();
    assert!(cancelling_ids.contains(&cancelling.id));
    assert!(cancelling_ids.contains(&finalizing.id));
    assert!(!cancelling_ids.contains(&pending.id));
    assert!(!cancelling_ids.contains(&paused.id));
    Ok(())
}
