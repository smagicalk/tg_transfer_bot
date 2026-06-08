// transfer_item 创建与引用计数相关测试。

use super::super::super::types::TransferBundle;
use super::super::*;
use super::fixtures::*;
use crate::db;
use rand::distr::SampleString;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

/// 创建子项时应在同一事务内增加文件引用，重复 ensure 不应重复计数。
#[tokio::test]
async fn test_ensure_items_for_bundle_acquires_file_ref_once() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_RUNNING).await?;
    let file_key = format!(
        "ensure_fk_{}",
        rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 16)
    );
    let message = message_with_document(123, 456, &file_key);

    let first = ensure_items_for_bundle(job.id, std::slice::from_ref(&message)).await?;
    let second = ensure_items_for_bundle(job.id, &[message]).await?;

    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 1);
    assert_eq!(first[0].id, second[0].id);

    let cache = db::file_cache::Entity::find_by_id(file_key)
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.active_refs, 1);
    assert!(cache.delete_after.is_none());
    Ok(())
}

/// 恢复重新 spider 后应按当前 bundle 对齐子项，并迁移/释放文件引用。
#[tokio::test]
async fn test_reconcile_items_for_bundle_updates_changed_and_missing_refs() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_RUNNING).await?;
    let old_key_a = unique_file_key("old_a");
    let old_key_b = unique_file_key("old_b");
    let new_key_a = unique_file_key("new_a");
    let new_key_d = unique_file_key("new_d");

    let old_a = message_with_document(123, 1, &old_key_a);
    let old_b = message_with_document(123, 2, &old_key_b);
    ensure_items_for_bundle(job.id, &[old_a, old_b]).await?;

    let new_a = message_with_document(123, 1, &new_key_a);
    let new_d = message_with_document(123, 4, &new_key_d);
    let bundle = TransferBundle {
        source_chat_id: 123,
        source_message_id: 4,
        source_album_id: 999,
        messages: vec![new_a, new_d],
    };

    reconcile_items_for_bundle(job.id, &bundle, 2).await?;

    let item_a = db::transfer_item::Entity::find()
        .filter(db::transfer_item::Column::JobId.eq(job.id))
        .filter(db::transfer_item::Column::SourceMessageId.eq(1))
        .one(db_conn)
        .await?
        .expect("changed item must exist");
    assert_eq!(item_a.file_key, new_key_a);
    assert_eq!(item_a.status, ITEM_STATUS_PENDING);
    assert!(!item_a.file_ref_released);

    let item_b = db::transfer_item::Entity::find()
        .filter(db::transfer_item::Column::JobId.eq(job.id))
        .filter(db::transfer_item::Column::SourceMessageId.eq(2))
        .one(db_conn)
        .await?
        .expect("missing item must be kept as obsolete");
    assert_eq!(item_b.status, ITEM_STATUS_OBSOLETE);
    assert!(item_b.file_ref_released);

    let item_d = db::transfer_item::Entity::find()
        .filter(db::transfer_item::Column::JobId.eq(job.id))
        .filter(db::transfer_item::Column::SourceMessageId.eq(4))
        .one(db_conn)
        .await?
        .expect("new item must be inserted");
    assert_eq!(item_d.file_key, new_key_d);
    assert!(!item_d.file_ref_released);

    assert_released_cache(db_conn, &old_key_a).await?;
    assert_released_cache(db_conn, &old_key_b).await?;
    assert_active_cache(db_conn, &new_key_a).await?;
    assert_active_cache(db_conn, &new_key_d).await?;

    let refreshed_job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(refreshed_job.source_message_id, 4);
    assert_eq!(refreshed_job.source_album_id, 999);
    assert_eq!(refreshed_job.total_items, 2);
    Ok(())
}

/// 最终释放应跳过已经由恢复对齐提前释放的子项，避免重复扣减共享引用。
#[tokio::test]
async fn test_release_job_file_refs_skips_already_released_items() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_RUNNING).await?;
    let shared_key = unique_file_key("shared");
    let changed_key = unique_file_key("changed");

    ensure_items_for_bundle(
        job.id,
        &[
            message_with_document(123, 1, &shared_key),
            message_with_document(123, 2, &changed_key),
        ],
    )
    .await?;
    acquire_file_ref(&shared_key).await?;

    let bundle = TransferBundle {
        source_chat_id: 123,
        source_message_id: 1,
        source_album_id: 0,
        messages: vec![message_with_document(123, 1, &shared_key)],
    };
    reconcile_items_for_bundle(job.id, &bundle, 2).await?;
    release_job_file_refs(job.id, 2).await?;

    let shared = db::file_cache::Entity::find_by_id(shared_key)
        .one(db_conn)
        .await?
        .expect("shared cache must exist");
    assert_eq!(shared.active_refs, 1);
    assert!(shared.delete_after.is_none());

    let changed = db::file_cache::Entity::find_by_id(changed_key)
        .one(db_conn)
        .await?
        .expect("changed cache must exist");
    assert_eq!(changed.active_refs, 0);
    assert!(changed.delete_after.is_some());
    Ok(())
}

/// 生成唯一 file_key，避免 SQLite 测试库复用时产生主键冲突。
fn unique_file_key(prefix: &str) -> String {
    format!(
        "{}_{}",
        prefix,
        rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 16)
    )
}

/// 断言缓存已没有活跃引用，并进入延迟删除队列。
async fn assert_released_cache(
    db_conn: &sea_orm::DatabaseConnection,
    file_key: &str,
) -> anyhow::Result<()> {
    let cache = db::file_cache::Entity::find_by_id(file_key.to_owned())
        .one(db_conn)
        .await?
        .expect("released cache must exist");
    assert_eq!(cache.active_refs, 0);
    assert!(cache.delete_after.is_some());
    Ok(())
}

/// 断言缓存仍被当前任务引用。
async fn assert_active_cache(
    db_conn: &sea_orm::DatabaseConnection,
    file_key: &str,
) -> anyhow::Result<()> {
    let cache = db::file_cache::Entity::find_by_id(file_key.to_owned())
        .one(db_conn)
        .await?
        .expect("active cache must exist");
    assert_eq!(cache.active_refs, 1);
    assert!(cache.delete_after.is_none());
    Ok(())
}
