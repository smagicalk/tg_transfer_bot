// file_cache 引用计数、取消收尾和 GC 删除队列相关测试。

use super::super::*;
use super::fixtures::*;
use crate::db;
use rand::distr::SampleString;
use sea_orm::{ActiveModelTrait, EntityTrait};

/// 立即停止任务时，未完成子项会变成 cancelled，文件引用会进入删除队列。
#[tokio::test]
async fn test_cancel_job_now_releases_file_refs() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_PENDING).await?;
    let (item_id, file_key) = insert_item_with_file_ref(job.id).await?;

    let cancelled = cancel_job_now(job.id, "cancelled by test", 2).await?;
    assert_eq!(cancelled.status, JOB_STATUS_CANCELLED);
    assert!(cancelled.finished_at.is_some());

    let item = db::transfer_item::Entity::find_by_id(item_id)
        .one(db_conn)
        .await?
        .expect("item must exist");
    assert_eq!(item.status, ITEM_STATUS_CANCELLED);

    let cache = db::file_cache::Entity::find_by_id(file_key)
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.active_refs, 0);
    assert!(cache.delete_after.is_some());
    Ok(())
}

/// 并发停止同一个任务时，文件引用只能释放一次，不能影响其他任务仍在使用的共享文件。
#[tokio::test]
async fn test_cancel_job_now_releases_file_refs_once_under_concurrency() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let now = now_utc8();
    let job1 = insert_job(JOB_STATUS_CANCELLING).await?;
    let job2 = insert_job(JOB_STATUS_RUNNING).await?;
    let file_key = format!(
        "fk_{}",
        rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 24)
    );

    db::file_cache::ActiveModel {
        file_key: sea_orm::ActiveValue::Set(file_key.clone()),
        status: sea_orm::ActiveValue::Set("ready".to_owned()),
        size_bytes: sea_orm::ActiveValue::Set(Some(2048)),
        td_file_id: sea_orm::ActiveValue::Set(Some(300)),
        local_path: sea_orm::ActiveValue::Set(Some("tmp/concurrent-cancel.bin".to_owned())),
        last_error: sea_orm::ActiveValue::Set(None),
        active_refs: sea_orm::ActiveValue::Set(2),
        last_ref_zero_at: sea_orm::ActiveValue::Set(None),
        delete_after: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        last_used_at: sea_orm::ActiveValue::Set(now),
    }
    .insert(db_conn)
    .await?;
    insert_item_for_file_key(job1.id, &file_key).await?;
    insert_item_for_file_key(job2.id, &file_key).await?;

    let (left, right) = tokio::join!(
        cancel_job_now(job1.id, "cancelled by test", 2),
        cancel_job_now(job1.id, "cancelled by test", 2)
    );
    left?;
    right?;

    let cache = db::file_cache::Entity::find_by_id(file_key)
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.active_refs, 1);
    assert!(cache.delete_after.is_none());
    Ok(())
}

/// acquire_file_ref 应使用数据库原子 upsert 增加引用，并取消旧删除计划。
#[tokio::test]
async fn test_acquire_file_ref_increments_and_clears_delete_plan() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let now = now_utc8();
    let file_key = format!(
        "fk_{}",
        rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 24)
    );

    db::file_cache::ActiveModel {
        file_key: sea_orm::ActiveValue::Set(file_key.clone()),
        status: sea_orm::ActiveValue::Set(FILE_CACHE_STATUS_DELETE_FAILED.to_owned()),
        size_bytes: sea_orm::ActiveValue::Set(Some(1024)),
        td_file_id: sea_orm::ActiveValue::Set(Some(100)),
        local_path: sea_orm::ActiveValue::Set(Some("tmp/test.bin".to_owned())),
        last_error: sea_orm::ActiveValue::Set(Some("old delete error".to_owned())),
        active_refs: sea_orm::ActiveValue::Set(0),
        last_ref_zero_at: sea_orm::ActiveValue::Set(Some(now)),
        delete_after: sea_orm::ActiveValue::Set(Some(now)),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        last_used_at: sea_orm::ActiveValue::Set(now),
    }
    .insert(db_conn)
    .await?;

    acquire_file_ref(&file_key).await?;
    acquire_file_ref(&file_key).await?;

    let cache = db::file_cache::Entity::find_by_id(file_key)
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.active_refs, 2);
    assert_eq!(cache.status, "pending");
    assert!(cache.last_ref_zero_at.is_none());
    assert!(cache.delete_after.is_none());
    assert!(cache.last_error.is_none());
    Ok(())
}

/// release_job_file_refs 应按任务扣减共享引用，不能把其他任务仍在用的文件提前入队删除。
#[tokio::test]
async fn test_release_job_file_refs_keeps_shared_file_until_last_ref() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let now = now_utc8();
    let job1 = insert_job(JOB_STATUS_RUNNING).await?;
    let job2 = insert_job(JOB_STATUS_RUNNING).await?;
    let file_key = format!(
        "fk_{}",
        rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 24)
    );

    db::file_cache::ActiveModel {
        file_key: sea_orm::ActiveValue::Set(file_key.clone()),
        status: sea_orm::ActiveValue::Set("ready".to_owned()),
        size_bytes: sea_orm::ActiveValue::Set(Some(2048)),
        td_file_id: sea_orm::ActiveValue::Set(Some(101)),
        local_path: sea_orm::ActiveValue::Set(Some("tmp/shared-test.bin".to_owned())),
        last_error: sea_orm::ActiveValue::Set(None),
        active_refs: sea_orm::ActiveValue::Set(2),
        last_ref_zero_at: sea_orm::ActiveValue::Set(None),
        delete_after: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        last_used_at: sea_orm::ActiveValue::Set(now),
    }
    .insert(db_conn)
    .await?;

    insert_item_for_file_key(job1.id, &file_key).await?;
    insert_item_for_file_key(job2.id, &file_key).await?;

    release_job_file_refs(job1.id, 2).await?;
    let cache = db::file_cache::Entity::find_by_id(file_key.clone())
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.active_refs, 1);
    assert!(cache.delete_after.is_none());

    release_job_file_refs(job2.id, 2).await?;
    let cache = db::file_cache::Entity::find_by_id(file_key)
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.active_refs, 0);
    assert!(cache.delete_after.is_some());
    Ok(())
}

/// GC 删除前必须先认领记录，认领后只能删除 status=deleting 且 active_refs=0 的行。
#[tokio::test]
async fn test_claim_and_delete_file_cache_for_gc() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let now = now_utc8();
    let file_key = format!(
        "fk_{}",
        rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 24)
    );

    db::file_cache::ActiveModel {
        file_key: sea_orm::ActiveValue::Set(file_key.clone()),
        status: sea_orm::ActiveValue::Set("ready".to_owned()),
        size_bytes: sea_orm::ActiveValue::Set(Some(2048)),
        td_file_id: sea_orm::ActiveValue::Set(Some(102)),
        local_path: sea_orm::ActiveValue::Set(Some("tmp/gc-test.bin".to_owned())),
        last_error: sea_orm::ActiveValue::Set(None),
        active_refs: sea_orm::ActiveValue::Set(0),
        last_ref_zero_at: sea_orm::ActiveValue::Set(Some(now)),
        delete_after: sea_orm::ActiveValue::Set(Some(now)),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        last_used_at: sea_orm::ActiveValue::Set(now),
    }
    .insert(db_conn)
    .await?;

    let claimed = claim_file_cache_for_delete(&file_key, now)
        .await?
        .expect("due cache should be claimed");
    assert_eq!(claimed.status, FILE_CACHE_STATUS_DELETING);
    assert!(claim_file_cache_for_delete(&file_key, now).await?.is_none());

    delete_file_cache(&file_key).await?;
    let cache = db::file_cache::Entity::find_by_id(file_key)
        .one(db_conn)
        .await?;
    assert!(cache.is_none());
    Ok(())
}

/// GC 删除失败时应延后下一次重试，避免同一坏路径在短间隔配置下被反复认领。
#[tokio::test]
async fn test_mark_file_cache_delete_failed_delays_retry() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let now = now_utc8();
    let retry_after = now + chrono::Duration::minutes(5);
    let file_key = format!(
        "fk_{}",
        rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 24)
    );

    db::file_cache::ActiveModel {
        file_key: sea_orm::ActiveValue::Set(file_key.clone()),
        status: sea_orm::ActiveValue::Set(FILE_CACHE_STATUS_DELETING.to_owned()),
        size_bytes: sea_orm::ActiveValue::Set(Some(2048)),
        td_file_id: sea_orm::ActiveValue::Set(Some(103)),
        local_path: sea_orm::ActiveValue::Set(Some("tmp/gc-failed.bin".to_owned())),
        last_error: sea_orm::ActiveValue::Set(None),
        active_refs: sea_orm::ActiveValue::Set(0),
        last_ref_zero_at: sea_orm::ActiveValue::Set(Some(now)),
        delete_after: sea_orm::ActiveValue::Set(Some(now)),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        last_used_at: sea_orm::ActiveValue::Set(now),
    }
    .insert(db_conn)
    .await?;

    mark_file_cache_delete_failed(&file_key, "delete failed by test".to_owned(), retry_after)
        .await?;

    let cache = db::file_cache::Entity::find_by_id(file_key.clone())
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.status, FILE_CACHE_STATUS_DELETE_FAILED);
    assert_eq!(cache.last_error.as_deref(), Some("delete failed by test"));
    assert_eq!(cache.delete_after, Some(retry_after));

    let due_rows = list_due_file_cache(now, 100).await?;
    assert!(!due_rows.iter().any(|row| row.file_key == file_key));
    Ok(())
}
