// 数据库模块：
// - 初始化 SeaORM 连接池
// - 定义 transfer_job / transfer_item / transfer_result_message / file_cache / menu_input_draft / user_account / point_ledger 实体
// - 运行时直接建表，不再维护独立 migration 历史
use sea_orm::sea_query::{ColumnDef, ForeignKeyAction, ForeignKeyCreateStatement, Index, Table};
use sea_orm::{ConnectionTrait, Database, StatementBuilder};
use std::path::{Path, PathBuf};
use std::time::Duration;

// 正常运行的默认 SQLite 数据库。
// 启动读取配置后会调用 `init_database_url` 覆盖为 config.json 的 `storage.database_url`。
#[cfg(not(test))]
const DATABASE_URL: &str = "sqlite://tg/app/transfer.sqlite?mode=rwc";

// 测试库会被直接删表重建，必须和正常运行库隔离。
#[cfg(test)]
const DATABASE_URL: &str = "sqlite://db.test.sqlite?mode=rwc";

// 全局数据库连接池句柄（惰性初始化）。
pub(crate) static DB_POOL: tokio::sync::OnceCell<sea_orm::DatabaseConnection> =
    tokio::sync::OnceCell::const_new();

// 运行期数据库连接串。
// 必须在第一次 `get_db()` 之前设置；测试环境保持固定测试库，避免被运行配置污染。
#[cfg(not(test))]
static DATABASE_URL_OVERRIDE: std::sync::OnceLock<String> = std::sync::OnceLock::new();

/// 初始化业务数据库连接串。
///
/// 业务数据库保存转存任务、子项和文件引用，不是 TDLib 账号数据库。
#[cfg(not(test))]
pub(crate) async fn init_database_url(database_url: impl Into<String>) -> anyhow::Result<()> {
    let database_url = database_url.into();
    if DB_POOL.initialized() {
        anyhow::bail!("database pool already initialized before database url was configured");
    }
    prepare_sqlite_parent_dir(&database_url).await?;
    DATABASE_URL_OVERRIDE
        .set(database_url)
        .map_err(|_| anyhow::anyhow!("database url already initialized"))?;
    Ok(())
}

/// 测试环境固定使用 `db.test.sqlite`，不允许运行配置改写。
#[cfg(test)]
pub(crate) async fn init_database_url(_database_url: impl Into<String>) -> anyhow::Result<()> {
    if DB_POOL.initialized() {
        anyhow::bail!("test database pool already initialized");
    }
    Ok(())
}

// 初始化数据库连接（SQLite）。
pub(crate) async fn init_db() -> anyhow::Result<sea_orm::DatabaseConnection> {
    let database_url = runtime_database_url();
    prepare_sqlite_parent_dir(database_url).await?;
    let mut opt = sea_orm::ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Info)
        .test_before_acquire(true)
        .connect_lazy(true);
    let db = Database::connect(opt).await?;
    Ok(db)
}

/// 返回当前生效的业务数据库连接串。
fn runtime_database_url() -> &'static str {
    #[cfg(not(test))]
    {
        DATABASE_URL_OVERRIDE
            .get()
            .map(String::as_str)
            .unwrap_or(DATABASE_URL)
    }

    #[cfg(test)]
    {
        DATABASE_URL
    }
}

/// SQLite 文件库需要先创建父目录，否则 `mode=rwc` 仍可能因为目录不存在而连接失败。
async fn prepare_sqlite_parent_dir(database_url: &str) -> anyhow::Result<()> {
    let Some(path) = sqlite_file_path(database_url) else {
        return Ok(());
    };
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        tokio::fs::create_dir_all(parent).await?;
    }
    Ok(())
}

/// 从 `sqlite://path?mode=rwc` 提取本地文件路径。
///
/// `sqlite::memory:`、`sqlite://:memory:` 和无文件路径的连接串不需要创建目录。
fn sqlite_file_path(database_url: &str) -> Option<PathBuf> {
    let path = database_url
        .strip_prefix("sqlite://")
        .or_else(|| database_url.strip_prefix("sqlite:"))?;
    let path = path.split('?').next().unwrap_or(path);
    if path.is_empty() || path == ":memory:" {
        return None;
    }
    Some(Path::new(path).to_path_buf())
}

// 获取数据库连接（必要时自动初始化）。
pub(crate) async fn get_db<'db>() -> anyhow::Result<&'db sea_orm::DatabaseConnection> {
    DB_POOL.get_or_try_init(init_db).await
}

/// 启动时确保业务表结构存在。
///
/// 当前仍处于开发期，业务 SQLite 可直接重建，因此这里不维护版本化 migration 历史，
/// 只保证“当前代码所需的完整表结构”存在。
pub(crate) async fn ensure_runtime_schema(db: &sea_orm::DatabaseConnection) -> anyhow::Result<()> {
    create_runtime_schema(db).await
}

/// 测试库结构自检。
///
/// 测试环境允许直接重建业务库；当发现旧测试库缺少当前代码依赖的列时，
/// 直接 drop + create，避免开发期 schema 演进把测试状态拖脏。
#[cfg(test)]
pub(crate) async fn ensure_test_schema_current(
    db: &sea_orm::DatabaseConnection,
) -> anyhow::Result<()> {
    create_runtime_schema(db).await?;
    if test_schema_has_required_columns(db).await? {
        return Ok(());
    }

    tracing::warn!("test database schema is stale, rebuilding test schema");
    rebuild_test_schema(db).await?;
    Ok(())
}

/// 测试环境直接删表重建。
#[cfg(test)]
pub(crate) async fn rebuild_test_schema(db: &sea_orm::DatabaseConnection) -> anyhow::Result<()> {
    drop_runtime_schema(db).await?;
    create_runtime_schema(db).await
}

/// 检查本轮测试依赖的关键列是否存在。
#[cfg(test)]
async fn test_schema_has_required_columns(
    db: &sea_orm::DatabaseConnection,
) -> anyhow::Result<bool> {
    Ok(
        sqlite_table_has_column(db, "transfer_job", "source_kind").await?
            && sqlite_table_has_column(db, "transfer_job", "owner_user_id").await?
            && sqlite_table_has_column(db, "transfer_job", "allow_user_fallback").await?
            && sqlite_table_has_column(db, "transfer_job", "billing_status").await?
            && sqlite_table_has_column(db, "transfer_item", "file_owner_client_role").await?
            && sqlite_table_has_column(db, "transfer_result_message", "message_link").await?
            && sqlite_table_has_column(db, "file_cache", "owner_client_role").await?
            && sqlite_table_has_column(db, "menu_input_draft", "expires_at").await?
            && sqlite_table_has_column(db, "user_account", "points_balance").await?
            && sqlite_table_has_column(db, "point_ledger", "idempotency_key").await?,
    )
}

/// SQLite PRAGMA table_info 只在测试库自检使用。
#[cfg(test)]
async fn sqlite_table_has_column(
    db: &sea_orm::DatabaseConnection,
    table: &str,
    column: &str,
) -> anyhow::Result<bool> {
    let sql = format!("PRAGMA table_info({})", table);
    let statement = sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Sqlite, sql);
    let rows = db.query_all_raw(statement).await?;
    for row in rows {
        let name: String = row.try_get("", "name")?;
        if name == column {
            return Ok(true);
        }
    }
    Ok(false)
}

/// 创建当前版本所需的全部业务表与索引。
async fn create_runtime_schema(db: &sea_orm::DatabaseConnection) -> anyhow::Result<()> {
    exec_schema_statement(
        db,
        Table::create()
            .table("user_account")
            .if_not_exists()
            .col(
                ColumnDef::new("telegram_user_id")
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new("role").string().not_null().default("user"))
            .col(
                ColumnDef::new("points_balance")
                    .big_integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new("total_points_added")
                    .big_integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new("total_points_spent")
                    .big_integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new("created_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new("updated_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("user_account_role_idx")
            .table("user_account")
            .col("role")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Table::create()
            .table("point_ledger")
            .if_not_exists()
            .col(
                ColumnDef::new("id")
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new("telegram_user_id").big_integer().not_null())
            .col(ColumnDef::new("delta").big_integer().not_null())
            .col(ColumnDef::new("balance_after").big_integer().not_null())
            .col(ColumnDef::new("reason").string().not_null())
            .col(ColumnDef::new("job_id").big_integer())
            .col(ColumnDef::new("request_chat_id").big_integer())
            .col(ColumnDef::new("request_message_id").big_integer())
            .col(ColumnDef::new("idempotency_key").string())
            .col(ColumnDef::new("created_by").big_integer())
            .col(
                ColumnDef::new("created_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("point_ledger_account_fk")
                    .from_tbl("point_ledger")
                    .from_col("telegram_user_id")
                    .to_tbl("user_account")
                    .to_col("telegram_user_id")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("point_ledger_idempotency_uk")
                    .col("idempotency_key")
                    .unique(),
            )
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("point_ledger_user_created_idx")
            .table("point_ledger")
            .col("telegram_user_id")
            .col("created_at")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Table::create()
            .table("transfer_job")
            .if_not_exists()
            .col(
                ColumnDef::new("id")
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new("request_chat_id").big_integer().not_null())
            .col(
                ColumnDef::new("request_message_id")
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new("owner_user_id")
                    .big_integer()
                    .not_null()
                    .default(0),
            )
            .col(ColumnDef::new("source_link").string().not_null())
            .col(
                ColumnDef::new("source_kind")
                    .string()
                    .not_null()
                    .default("link"),
            )
            .col(
                ColumnDef::new("source_client_role")
                    .string()
                    .not_null()
                    .default("user"),
            )
            .col(
                ColumnDef::new("allow_user_fallback")
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(ColumnDef::new("source_chat_id").big_integer().not_null())
            .col(ColumnDef::new("source_message_id").big_integer().not_null())
            .col(
                ColumnDef::new("source_album_id")
                    .big_integer()
                    .not_null()
                    .default(0),
            )
            .col(ColumnDef::new("target_chat_id").big_integer().not_null())
            .col(ColumnDef::new("result_message_id").big_integer())
            .col(ColumnDef::new("result_message_link").string())
            .col(ColumnDef::new("status").string().not_null())
            .col(
                ColumnDef::new("total_items")
                    .integer()
                    .not_null()
                    .default(0),
            )
            .col(ColumnDef::new("done_items").integer().not_null().default(0))
            .col(
                ColumnDef::new("failed_items")
                    .integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new("retry_count")
                    .integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new("cost_points")
                    .big_integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new("charged_points")
                    .big_integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new("billing_status")
                    .string()
                    .not_null()
                    .default("free"),
            )
            .col(ColumnDef::new("last_error").string())
            .col(
                ColumnDef::new("created_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new("updated_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new("finished_at").timestamp_with_time_zone())
            .index(
                Index::create()
                    .name("transfer_job_request_uk")
                    .col("request_chat_id")
                    .col("request_message_id")
                    .unique(),
            )
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("transfer_job_status_updated_idx")
            .table("transfer_job")
            .col("status")
            .col("updated_at")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("transfer_job_owner_status_updated_idx")
            .table("transfer_job")
            .col("owner_user_id")
            .col("status")
            .col("updated_at")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("transfer_job_source_link_idx")
            .table("transfer_job")
            .col("source_link")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("transfer_job_source_target_status_idx")
            .table("transfer_job")
            .col("source_link")
            .col("target_chat_id")
            .col("status")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Table::create()
            .table("transfer_item")
            .if_not_exists()
            .col(
                ColumnDef::new("id")
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new("job_id").big_integer().not_null())
            .col(ColumnDef::new("source_chat_id").big_integer().not_null())
            .col(ColumnDef::new("source_message_id").big_integer().not_null())
            .col(ColumnDef::new("file_key").string().not_null())
            .col(
                ColumnDef::new("file_owner_client_role")
                    .string()
                    .not_null()
                    .default("user"),
            )
            .col(ColumnDef::new("status").string().not_null())
            .col(
                ColumnDef::new("retry_count")
                    .integer()
                    .not_null()
                    .default(0),
            )
            .col(ColumnDef::new("error_message").string())
            .col(
                ColumnDef::new("file_ref_released")
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(
                ColumnDef::new("created_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new("updated_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("transfer_item_job_fk")
                    .from_tbl("transfer_item")
                    .from_col("job_id")
                    .to_tbl("transfer_job")
                    .to_col("id")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("transfer_item_job_source_uk")
                    .col("job_id")
                    .col("source_chat_id")
                    .col("source_message_id")
                    .unique(),
            )
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("transfer_item_job_status_idx")
            .table("transfer_item")
            .col("job_id")
            .col("status")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Table::create()
            .table("transfer_result_message")
            .if_not_exists()
            .col(
                ColumnDef::new("id")
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new("job_id").big_integer().not_null())
            .col(ColumnDef::new("result_index").integer().not_null())
            .col(ColumnDef::new("target_chat_id").big_integer().not_null())
            .col(ColumnDef::new("message_id").big_integer().not_null())
            .col(ColumnDef::new("message_link").string().not_null())
            .col(
                ColumnDef::new("is_album")
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(ColumnDef::new("item_count").integer().not_null().default(1))
            .col(
                ColumnDef::new("created_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new("updated_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("transfer_result_message_job_fk")
                    .from_tbl("transfer_result_message")
                    .from_col("job_id")
                    .to_tbl("transfer_job")
                    .to_col("id")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("transfer_result_message_job_index_uk")
                    .col("job_id")
                    .col("result_index")
                    .unique(),
            )
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("transfer_result_message_job_idx")
            .table("transfer_result_message")
            .col("job_id")
            .col("result_index")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Table::create()
            .table("file_cache")
            .if_not_exists()
            .col(
                ColumnDef::new("owner_client_role")
                    .string()
                    .not_null()
                    .default("user"),
            )
            .col(ColumnDef::new("file_key").string().not_null())
            .col(ColumnDef::new("status").string().not_null())
            .col(ColumnDef::new("size_bytes").big_integer())
            .col(ColumnDef::new("td_file_id").integer())
            .col(ColumnDef::new("local_path").string())
            .col(ColumnDef::new("last_error").string())
            .col(
                ColumnDef::new("active_refs")
                    .integer()
                    .not_null()
                    .default(0),
            )
            .col(ColumnDef::new("last_ref_zero_at").timestamp_with_time_zone())
            .col(ColumnDef::new("delete_after").timestamp_with_time_zone())
            .col(
                ColumnDef::new("created_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new("updated_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new("last_used_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .primary_key(Index::create().col("owner_client_role").col("file_key"))
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("file_cache_status_last_used_idx")
            .table("file_cache")
            .col("status")
            .col("last_used_at")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("file_cache_gc_due_idx")
            .table("file_cache")
            .col("active_refs")
            .col("delete_after")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Table::create()
            .table("menu_input_draft")
            .if_not_exists()
            .col(ColumnDef::new("request_chat_id").big_integer().not_null())
            .col(ColumnDef::new("sender_user_id").big_integer().not_null())
            .col(ColumnDef::new("step").string().not_null())
            .col(ColumnDef::new("input_kind").string())
            .col(ColumnDef::new("job_action").string())
            .col(ColumnDef::new("source_link").string())
            .col(ColumnDef::new("target_chat_id").big_integer())
            .col(
                ColumnDef::new("created_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new("updated_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new("expires_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .primary_key(Index::create().col("request_chat_id").col("sender_user_id"))
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("menu_input_draft_expires_idx")
            .table("menu_input_draft")
            .col("expires_at")
            .to_owned(),
    )
    .await?;

    Ok(())
}

/// 执行单条 schema builder。
async fn exec_schema_statement<S>(
    db: &sea_orm::DatabaseConnection,
    statement: S,
) -> anyhow::Result<()>
where
    S: StatementBuilder,
{
    db.execute(&statement).await?;
    Ok(())
}

/// 测试环境按依赖反序删表。
#[cfg(test)]
async fn drop_runtime_schema(db: &sea_orm::DatabaseConnection) -> anyhow::Result<()> {
    exec_schema_statement(
        db,
        Table::drop()
            .table("menu_input_draft")
            .if_exists()
            .to_owned(),
    )
    .await?;
    exec_schema_statement(db, Table::drop().table("file_cache").if_exists().to_owned()).await?;
    exec_schema_statement(
        db,
        Table::drop()
            .table("transfer_result_message")
            .if_exists()
            .to_owned(),
    )
    .await?;
    exec_schema_statement(
        db,
        Table::drop().table("transfer_item").if_exists().to_owned(),
    )
    .await?;
    exec_schema_statement(
        db,
        Table::drop().table("transfer_job").if_exists().to_owned(),
    )
    .await?;
    exec_schema_statement(
        db,
        Table::drop().table("point_ledger").if_exists().to_owned(),
    )
    .await?;
    exec_schema_statement(
        db,
        Table::drop().table("user_account").if_exists().to_owned(),
    )
    .await?;
    Ok(())
}

// DB 测试共用同一个 SQLite 文件与全局连接池。
// 为避免重建表结构与插入测试并发互相影响，这里串行执行。
#[cfg(test)]
pub(crate) static TEST_DB_LOCK: std::sync::LazyLock<tokio::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| tokio::sync::Mutex::new(()));

pub(crate) mod file_cache;
pub(crate) mod menu_input_draft;
pub(crate) mod point_ledger;
pub(crate) mod transfer_item;
pub(crate) mod transfer_job;
pub(crate) mod transfer_result_message;
pub(crate) mod user_account;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod url_tests {
    use super::*;

    #[test]
    fn test_sqlite_file_path_extracts_file_path() {
        assert_eq!(
            sqlite_file_path("sqlite://tg/app/transfer.sqlite?mode=rwc"),
            Some(PathBuf::from("tg/app/transfer.sqlite"))
        );
        assert_eq!(
            sqlite_file_path("sqlite:relative.sqlite"),
            Some(PathBuf::from("relative.sqlite"))
        );
    }

    #[test]
    fn test_sqlite_file_path_ignores_memory_or_non_sqlite() {
        assert_eq!(sqlite_file_path("sqlite::memory:"), None);
        assert_eq!(sqlite_file_path("sqlite://:memory:"), None);
        assert_eq!(sqlite_file_path("postgres://localhost/db"), None);
    }
}
