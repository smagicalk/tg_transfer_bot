// 数据库模块测试：
// - schema create / rebuild
// - 基础插入
// - request 级任务创建语义
// - file_cache 去重插入

use super::*;
use crate::logs::init_tracing;
use rand::RngExt;
use rand::distr::SampleString;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter};
use sea_orm::{ColumnTrait, ConnectionTrait};
use std::sync::LazyLock;

#[cfg(test)]
static POSTGRES_TEST_LOCK: LazyLock<tokio::sync::Mutex<()>> =
    LazyLock::new(|| tokio::sync::Mutex::new(()));

/// 统一生成 UTC+8 时间。
fn now_utc8() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap())
}

/// 测试前确保业务 schema 已经是当前版本。
async fn prepare_test_schema() -> anyhow::Result<&'static sea_orm::DatabaseConnection> {
    let db = get_db().await?;
    super::ensure_test_schema_current(db).await?;
    Ok(db)
}

fn test_postgres_database_url() -> Option<String> {
    std::env::var("TEST_POSTGRES_DATABASE_URL")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn unique_pg_schema_name() -> String {
    format!(
        "tg_transfer_test_{}",
        rand::distr::Alphanumeric
            .sample_string(&mut rand::rng(), 12)
            .to_lowercase()
    )
}

async fn connect_postgres_test_db() -> anyhow::Result<Option<(sea_orm::DatabaseConnection, String)>>
{
    let Some(database_url) = test_postgres_database_url() else {
        return Ok(None);
    };
    let schema = unique_pg_schema_name();
    let connect_url = if database_url.contains('?') {
        format!("{database_url}&search_path={schema}")
    } else {
        format!("{database_url}?search_path={schema}")
    };

    let db = sea_orm::Database::connect(connect_url).await?;
    db.execute_unprepared(&format!(r#"CREATE SCHEMA IF NOT EXISTS "{schema}""#))
        .await?;
    Ok(Some((db, schema)))
}

async fn drop_postgres_test_schema(
    db: &sea_orm::DatabaseConnection,
    schema: &str,
) -> anyhow::Result<()> {
    db.execute_unprepared(&format!(r#"DROP SCHEMA IF EXISTS "{schema}" CASCADE"#))
        .await?;
    Ok(())
}

/// 构造最小默认运行态，供真实启动数据库链测试复用。
fn test_bootstrap_defaults() -> (crate::config::TransferConfig, crate::config::TargetsConfig) {
    (
        crate::config::TransferConfig {
            job_concurrency: 4,
            file_delete_delay_minutes: 6,
            file_gc_interval_seconds: 120,
            progress_edit_interval_seconds: 7,
            downloads_default_page_size: 9,
            menu_input_timeout_seconds: 800,
        },
        crate::config::TargetsConfig {
            default_chat_id: -1001234567890,
            aliases: std::collections::HashMap::from([("archive".to_owned(), -1001234567890)]),
        },
    )
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

/// migration 执行后应包含成功结果复用索引。
#[tokio::test]
async fn test_migrator_creates_success_lookup_index() -> anyhow::Result<()> {
    let _guard = super::TEST_DB_LOCK.lock().await;
    init_tracing();
    let db = get_db().await?;
    rebuild_test_schema(db).await?;
    super::ensure_runtime_schema(db).await?;

    let index_exists = match db.get_database_backend() {
        sea_orm::DatabaseBackend::Sqlite => {
            let statement = sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                "PRAGMA index_list('transfer_job')".to_owned(),
            );
            let rows = db.query_all_raw(statement).await?;
            rows.into_iter().any(|row| {
                row.try_get::<String>("", "name")
                    .is_ok_and(|name| name == "transfer_job_success_lookup_idx")
            })
        }
        sea_orm::DatabaseBackend::Postgres => {
            let statement = super::raw_statement_for_backend(
                sea_orm::DatabaseBackend::Postgres,
                "",
                r#"
                SELECT 1
                FROM pg_indexes
                WHERE schemaname = current_schema()
                  AND tablename = $1
                  AND indexname = $2
                LIMIT 1
                "#,
                vec![
                    "transfer_job".into(),
                    "transfer_job_success_lookup_idx".into(),
                ],
            )?;
            !db.query_all_raw(statement).await?.is_empty()
        }
        backend => anyhow::bail!("unsupported database backend for index probe: {backend:?}"),
    };

    assert!(index_exists);
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
    Ok(())
}

/// 动态授权应持久化且保持幂等，供运行时命令安全重试。
#[tokio::test]
async fn test_authorized_user_persistence_lifecycle() -> anyhow::Result<()> {
    let _guard = super::TEST_DB_LOCK.lock().await;
    init_tracing();
    let db = prepare_test_schema().await?;
    let user_id = rand::rng().random_range(1_000_000..=9_999_999);

    crate::access::revoke_authorized_user_on(db, user_id).await?;
    assert!(crate::access::grant_authorized_user_on(db, user_id).await?);
    assert!(!crate::access::grant_authorized_user_on(db, user_id).await?);
    assert!(
        crate::access::list_authorized_user_ids_on(db)
            .await?
            .contains(&user_id)
    );

    assert!(crate::access::revoke_authorized_user_on(db, user_id).await?);
    assert!(!crate::access::revoke_authorized_user_on(db, user_id).await?);
    assert!(
        !crate::access::list_authorized_user_ids_on(db)
            .await?
            .contains(&user_id)
    );
    Ok(())
}

/// 动态授权应保存可选的 Telegram 名称资料，列表读取后可用于管理员界面展示。
#[tokio::test]
async fn test_authorized_user_profile_lifecycle() -> anyhow::Result<()> {
    let _guard = super::TEST_DB_LOCK.lock().await;
    init_tracing();
    let db = prepare_test_schema().await?;
    let user_id = rand::rng().random_range(300_000_000..=399_999_999);

    crate::access::revoke_authorized_user_on(db, user_id).await?;
    assert!(
        crate::access::grant_authorized_user_with_profile_on(
            db,
            user_id,
            Some("张三"),
            Some("zhangsan"),
        )
        .await?
    );

    let users = crate::access::list_authorized_users_on(db).await?;
    let user = users
        .iter()
        .find(|user| user.user_id == user_id)
        .expect("profile row should be listed");
    assert_eq!(user.display_name.as_deref(), Some("张三"));
    assert_eq!(user.username.as_deref(), Some("zhangsan"));

    assert!(crate::access::update_authorized_user_profile_on(
        db,
        user_id,
        Some("张三（管理员）"),
        None,
    )
    .await?);
    let user = crate::access::list_authorized_users_on(db)
        .await?
        .into_iter()
        .find(|user| user.user_id == user_id)
        .expect("updated profile row should be listed");
    assert_eq!(user.display_name.as_deref(), Some("张三（管理员）"));
    assert_eq!(user.username, None);

    assert!(
        !crate::access::update_authorized_user_profile_on(db, user_id + 1, Some("不存在"), None,)
            .await?
    );
    crate::access::revoke_authorized_user_on(db, user_id).await?;
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

/// SQLite 启动链验证：
/// - 走和 `run()` 相同的数据库 bootstrap helper
/// - 确认 migration 后两类运行态 seed 都会落库并可回读
#[tokio::test]
async fn test_runtime_bootstrap_helper_seeds_sqlite_runtime_state() -> anyhow::Result<()> {
    let _guard = super::TEST_DB_LOCK.lock().await;
    init_tracing();
    let db = get_db().await?;
    rebuild_test_schema(db).await?;
    let authorized_user_id = 8_765_432;
    crate::access::grant_authorized_user_on(db, authorized_user_id).await?;

    let (transfer_default, targets_default) = test_bootstrap_defaults();
    let seeded = crate::bootstrap_runtime_database_state_on(
        db,
        super::connection::runtime_database_url(),
        &transfer_default,
        &targets_default,
    )
    .await?;

    assert_eq!(seeded.transfer_config.job_concurrency, 4);
    assert_eq!(seeded.transfer_config.file_delete_delay_minutes, 6);
    assert_eq!(seeded.targets_config.default_chat_id, -1001234567890);
    assert_eq!(
        seeded.targets_config.aliases.get("archive"),
        Some(&-1001234567890)
    );
    assert!(seeded.authorized_user_ids.contains(&authorized_user_id));

    let runtime_row = crate::tgbot::transfer::load_transfer_runtime_config()
        .await?
        .expect("runtime config row");
    assert_eq!(runtime_row.job_concurrency, 4);
    assert_eq!(runtime_row.menu_input_timeout_seconds, 800);
    Ok(())
}

/// PostgreSQL 路径验证：
/// - 独立 schema 创建
/// - 走真实启动数据库链（migration + 两类运行态 seed）
/// - 关键列探测
/// - 基础插入
///
/// 仅当设置 `TEST_POSTGRES_DATABASE_URL` 时执行；默认开发环境仍只跑 SQLite。
#[tokio::test]
async fn test_postgres_migration_and_insert_when_env_is_present() -> anyhow::Result<()> {
    let _guard = POSTGRES_TEST_LOCK.lock().await;
    init_tracing();
    let Some((db, schema)) = connect_postgres_test_db().await? else {
        return Ok(());
    };

    let result = async {
        let (transfer_default, targets_default) = test_bootstrap_defaults();
        let seeded = crate::bootstrap_runtime_database_state_on(
            &db,
            &format!("postgresql://<hidden>?search_path={schema}"),
            &transfer_default,
            &targets_default,
        )
        .await?;

        assert!(test_schema_has_required_columns(&db).await?);
        assert_eq!(seeded.transfer_config.job_concurrency, 4);
        assert_eq!(seeded.targets_config.default_chat_id, -1001234567890);

        let job = get_transfer_job().await.insert(&db).await?;
        transfer_item::Entity::insert(get_transfer_item(job.id).await)
            .on_conflict_do_nothing()
            .exec(&db)
            .await?;
        menu_input_draft::Entity::insert(get_menu_input_draft().await)
            .on_conflict_do_nothing()
            .exec(&db)
            .await?;

        let inserted_jobs = transfer_job::Entity::find().all(&db).await?;
        assert_eq!(inserted_jobs.len(), 1);
        let runtime_row = crate::tgbot::transfer::load_transfer_runtime_config_on(&db)
            .await?
            .expect("runtime config row");
        assert_eq!(runtime_row.job_concurrency, 4);
        anyhow::Ok(())
    }
    .await;

    let cleanup_result = drop_postgres_test_schema(&db, &schema).await;
    db.close().await?;

    result?;
    cleanup_result?;
    Ok(())
}
