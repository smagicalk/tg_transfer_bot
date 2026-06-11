// 数据库模块测试：
// - schema create / rebuild
// - 基础插入
// - request 级任务创建语义
// - file_cache 去重插入

use super::*;
use crate::logs::init_tracing;
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
    super::ensure_test_schema_current(db).await?;
    Ok(db)
}

/// 构造 transfer_job 测试数据。
async fn get_transfer_job() -> transfer_job::ActiveModel {
    let now = now_utc8();
    transfer_job::ActiveModel {
        request_chat_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        request_message_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        owner_user_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        source_link: sea_orm::ActiveValue::set(
            rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 32),
        ),
        source_kind: sea_orm::ActiveValue::set("link".to_owned()),
        source_client_role: sea_orm::ActiveValue::set("user".to_owned()),
        allow_user_fallback: sea_orm::ActiveValue::set(false),
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
        cost_points: sea_orm::ActiveValue::set(0),
        charged_points: sea_orm::ActiveValue::set(0),
        billing_status: sea_orm::ActiveValue::set("free".to_owned()),
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
        file_owner_client_role: sea_orm::ActiveValue::set("user".to_owned()),
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
        owner_client_role: sea_orm::ActiveValue::set("user".to_owned()),
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

/// 构造 menu_input_draft 测试数据。
async fn get_menu_input_draft() -> menu_input_draft::ActiveModel {
    let now = now_utc8();
    menu_input_draft::ActiveModel {
        request_chat_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        sender_user_id: sea_orm::ActiveValue::set(rand::rng().random_range(1..=100000)),
        step: sea_orm::ActiveValue::set("source_link".to_owned()),
        input_kind: sea_orm::ActiveValue::set(Some("transfer".to_owned())),
        job_action: sea_orm::ActiveValue::set(None),
        source_link: sea_orm::ActiveValue::set(None),
        target_chat_id: sea_orm::ActiveValue::set(None),
        created_at: sea_orm::ActiveValue::set(now),
        updated_at: sea_orm::ActiveValue::set(now),
        expires_at: sea_orm::ActiveValue::set(now + chrono::Duration::minutes(10)),
    }
}

/// 构造 user_account 测试数据。
async fn get_user_account(telegram_user_id: i64) -> user_account::ActiveModel {
    let now = now_utc8();
    user_account::ActiveModel {
        telegram_user_id: sea_orm::ActiveValue::set(telegram_user_id),
        role: sea_orm::ActiveValue::set("user".to_owned()),
        points_balance: sea_orm::ActiveValue::set(10),
        total_points_added: sea_orm::ActiveValue::set(10),
        total_points_spent: sea_orm::ActiveValue::set(0),
        created_at: sea_orm::ActiveValue::set(now),
        updated_at: sea_orm::ActiveValue::set(now),
    }
}

#[tokio::test]
async fn test_schema_create() -> anyhow::Result<()> {
    let _guard = super::TEST_DB_LOCK.lock().await;
    init_tracing();
    ensure_runtime_schema(get_db().await?).await?;
    Ok(())
}

#[tokio::test]
async fn test_schema_rebuild() -> anyhow::Result<()> {
    let _guard = super::TEST_DB_LOCK.lock().await;
    init_tracing();
    let db = get_db().await?;
    ensure_runtime_schema(db).await?;
    rebuild_test_schema(db).await?;
    assert!(test_schema_has_required_columns(db).await?);
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
    menu_input_draft::Entity::insert(get_menu_input_draft().await)
        .on_conflict_do_nothing()
        .exec(db)
        .await?;
    let telegram_user_id = rand::rng().random_range(1..=100000);
    user_account::Entity::insert(get_user_account(telegram_user_id).await)
        .exec(db)
        .await?;
    point_ledger::ActiveModel {
        telegram_user_id: sea_orm::ActiveValue::set(telegram_user_id),
        delta: sea_orm::ActiveValue::set(10),
        balance_after: sea_orm::ActiveValue::set(10),
        reason: sea_orm::ActiveValue::set("test".to_owned()),
        job_id: sea_orm::ActiveValue::set(None),
        request_chat_id: sea_orm::ActiveValue::set(None),
        request_message_id: sea_orm::ActiveValue::set(None),
        idempotency_key: sea_orm::ActiveValue::set(None),
        created_by: sea_orm::ActiveValue::set(None),
        created_at: sea_orm::ActiveValue::set(now_utc8()),
        ..Default::default()
    }
    .insert(db)
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
        owner_user_id: sea_orm::ActiveValue::set(request_chat_id1),
        source_link: sea_orm::ActiveValue::set(source_link.clone()),
        source_kind: sea_orm::ActiveValue::set("link".to_owned()),
        source_client_role: sea_orm::ActiveValue::set("user".to_owned()),
        allow_user_fallback: sea_orm::ActiveValue::set(false),
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
        cost_points: sea_orm::ActiveValue::set(0),
        charged_points: sea_orm::ActiveValue::set(0),
        billing_status: sea_orm::ActiveValue::set("free".to_owned()),
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
        owner_user_id: sea_orm::ActiveValue::set(request_chat_id2),
        source_link: sea_orm::ActiveValue::set(source_link),
        source_kind: sea_orm::ActiveValue::set("link".to_owned()),
        source_client_role: sea_orm::ActiveValue::set("user".to_owned()),
        allow_user_fallback: sea_orm::ActiveValue::set(false),
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
        cost_points: sea_orm::ActiveValue::set(0),
        charged_points: sea_orm::ActiveValue::set(0),
        billing_status: sea_orm::ActiveValue::set("free".to_owned()),
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
