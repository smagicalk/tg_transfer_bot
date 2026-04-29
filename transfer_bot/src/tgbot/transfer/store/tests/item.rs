// transfer_item 创建与引用计数相关测试。

use super::super::*;
use super::fixtures::*;
use crate::db;
use rand::distr::SampleString;
use sea_orm::EntityTrait;

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
