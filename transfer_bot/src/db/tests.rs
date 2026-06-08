// 数据库模块测试：
// - migration up/down
// - 基础插入
// - request 级任务创建语义
// - file_cache 去重插入

use super::*;
use crate::logs::init_tracing;
use migration::MigratorTrait;
use rand::RngExt;
use rand::distr::SampleString;
use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter};

/// 统一生成 UTC+8 时间。
fn now_utc8() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap())
}

/// 测试前确保表结构已经准备好。
async fn prepare_test_schema() -> anyhow::Result<&'static sea_orm::DatabaseConnection> {
    let db = get_db().await?;
    migration::Migrator::up(db, None).await?;
    Ok(db)
}

/// 构造 transfer_job 测试数据。
async fn get_transfer_job() -> transfer_job::ActiveModel {
    let now = now_utc8();
    transfer_job::ActiveModel {
        request_chat_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        request_message_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        source_link: sea_orm::ActiveValue::set(
            rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 32),
        ),
        source_chat_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        source_message_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        source_album_id: sea_orm::ActiveValue::set(rand::rng().random_range(0..=100000)),
        target_chat_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        result_message_id: sea_orm::ActiveValue::set(None),
        result_message_link: sea_orm::ActiveValue::set(None),
        status: sea_orm::ActiveValue::set("pending".to_owned()),
        total_items: sea_orm::ActiveValue::set(1),
        done_items: sea_orm::ActiveValue::set(0),
        failed_items: sea_orm::ActiveValue::set(0),
        retry_count: sea_orm::ActiveValue::set(0),
        last_error: sea_orm::ActiveValue::set(None),
        created_at: sea_orm::ActiveValue::set(now),
        updated_at: sea_orm::ActiveValue::set(now),
        finished_at: sea_orm::ActiveValue::set(None),
        ..Default::default()
    }
}

/// 构造 transfer_item 测试数据。
async fn get_transfer_item(job_id: i64) -> transfer_item::ActiveModel {
    let now = now_utc8();
    transfer_item::ActiveModel {
        job_id: sea_orm::ActiveValue::set(job_id),
        source_chat_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        source_message_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        file_key: sea_orm::ActiveValue::set(format!(
            "fk_{}",
            rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 16)
        )),
        status: sea_orm::ActiveValue::set("pending".to_owned()),
        retry_count: sea_orm::ActiveValue::set(0),
        error_message: sea_orm::ActiveValue::set(None),
        created_at: sea_orm::ActiveValue::set(now),
        updated_at: sea_orm::ActiveValue::set(now),
        ..Default::default()
    }
}

/// 构造 file_cache 测试数据。
async fn get_file_cache(file_key: String) -> file_cache::ActiveModel {
    let now = now_utc8();
    file_cache::ActiveModel {
        file_key: sea_orm::ActiveValue::set(file_key),
        status: sea_orm::ActiveValue::set("downloading".to_owned()),
        size_bytes: sea_orm::ActiveValue::set(None),
        td_file_id: sea_orm::ActiveValue::set(None),
        local_path: sea_orm::ActiveValue::set(None),
        last_error: sea_orm::ActiveValue::set(None),
        active_refs: sea_orm::ActiveValue::set(1),
        last_ref_zero_at: sea_orm::ActiveValue::set(None),
        delete_after: sea_orm::ActiveValue::set(None),
        created_at: sea_orm::ActiveValue::set(now),
        updated_at: sea_orm::ActiveValue::set(now),
        last_used_at: sea_orm::ActiveValue::set(now),
    }
}

#[tokio::test]
async fn test_migration_up() -> anyhow::Result<()> {
    let _guard = super::TEST_DB_LOCK.lock().await;
    init_tracing();
    migration::Migrator::up(get_db().await?, None).await?;
    Ok(())
}

#[tokio::test]
async fn test_migration_down() -> anyhow::Result<()> {
    let _guard = super::TEST_DB_LOCK.lock().await;
    init_tracing();
    migration::Migrator::up(get_db().await?, None).await?;
    migration::Migrator::down(get_db().await?, None).await?;
    Ok(())
}

#[tokio::test]
async fn test_insert() -> anyhow::Result<()> {
    let _guard = super::TEST_DB_LOCK.lock().await;
    init_tracing();
    let db = prepare_test_schema().await?;
    let job = get_transfer_job().await.insert(db).await?;
    let item = get_transfer_item(job.id).await;
    transfer_item::Entity::insert(item)
        .on_conflict_do_nothing()
        .exec(db)
        .await?;
    Ok(())
}

/// 同一个 source_link 可以创建不同 job（不同请求消息）。
#[tokio::test]
async fn test_same_link_can_create_different_jobs() -> anyhow::Result<()> {
    let _guard = super::TEST_DB_LOCK.lock().await;
    init_tracing();
    let db = prepare_test_schema().await?;
    let now = now_utc8();
    let source_link = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 32);
    let source_chat_id = rand::rng().random_range(1..=100000);
    let source_message_id = rand::rng().random_range(1..=100000);
    let target_chat_id = rand::rng().random_range(1..=100000);

    let request_chat_id1 = rand::rng().random_range(1..=100000);
    let request_chat_id2 = request_chat_id1 + 100000;
    let request_message_id1 = rand::rng().random_range(1..=100000);
    let request_message_id2 = request_message_id1 + 100000;

    let job1 = transfer_job::ActiveModel {
        request_chat_id: sea_orm::ActiveValue::set(request_chat_id1),
        request_message_id: sea_orm::ActiveValue::set(request_message_id1),
        source_link: sea_orm::ActiveValue::set(source_link.clone()),
        source_chat_id: sea_orm::ActiveValue::set(source_chat_id),
        source_message_id: sea_orm::ActiveValue::set(source_message_id),
        source_album_id: sea_orm::ActiveValue::set(0),
        target_chat_id: sea_orm::ActiveValue::set(target_chat_id),
        result_message_id: sea_orm::ActiveValue::set(None),
        result_message_link: sea_orm::ActiveValue::set(None),
        status: sea_orm::ActiveValue::set("pending".to_owned()),
        total_items: sea_orm::ActiveValue::set(1),
        done_items: sea_orm::ActiveValue::set(0),
        failed_items: sea_orm::ActiveValue::set(0),
        retry_count: sea_orm::ActiveValue::set(0),
        last_error: sea_orm::ActiveValue::set(None),
        created_at: sea_orm::ActiveValue::set(now),
        updated_at: sea_orm::ActiveValue::set(now),
        finished_at: sea_orm::ActiveValue::set(None),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let job2 = transfer_job::ActiveModel {
        request_chat_id: sea_orm::ActiveValue::set(request_chat_id2),
        request_message_id: sea_orm::ActiveValue::set(request_message_id2),
        source_link: sea_orm::ActiveValue::set(source_link),
        source_chat_id: sea_orm::ActiveValue::set(source_chat_id),
        source_message_id: sea_orm::ActiveValue::set(source_message_id),
        source_album_id: sea_orm::ActiveValue::set(0),
        target_chat_id: sea_orm::ActiveValue::set(target_chat_id),
        result_message_id: sea_orm::ActiveValue::set(None),
        result_message_link: sea_orm::ActiveValue::set(None),
        status: sea_orm::ActiveValue::set("pending".to_owned()),
        total_items: sea_orm::ActiveValue::set(1),
        done_items: sea_orm::ActiveValue::set(0),
        failed_items: sea_orm::ActiveValue::set(0),
        retry_count: sea_orm::ActiveValue::set(0),
        last_error: sea_orm::ActiveValue::set(None),
        created_at: sea_orm::ActiveValue::set(now),
        updated_at: sea_orm::ActiveValue::set(now),
        finished_at: sea_orm::ActiveValue::set(None),
        ..Default::default()
    }
    .insert(db)
    .await?;

    assert!(job1.id != job2.id);
    Ok(())
}

/// 相同 file_key 重复插入时，只保留一行。
#[tokio::test]
async fn test_file_cache_dedup_insert() -> anyhow::Result<()> {
    let _guard = super::TEST_DB_LOCK.lock().await;
    init_tracing();
    let db = prepare_test_schema().await?;
    let file_key = format!(
        "fk_{}",
        rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 20)
    );

    let first = get_file_cache(file_key.clone()).await;
    file_cache::Entity::insert(first)
        .on_conflict_do_nothing()
        .exec(db)
        .await?;

    let second = get_file_cache(file_key.clone()).await;
    file_cache::Entity::insert(second)
        .on_conflict_do_nothing()
        .exec(db)
        .await?;

    let rows = file_cache::Entity::find()
        .filter(file_cache::Column::FileKey.eq(file_key))
        .all(db)
        .await?;
    assert_eq!(rows.len(), 1);
    Ok(())
}
