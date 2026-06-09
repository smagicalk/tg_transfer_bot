// 数据库模块：
// - 初始化 SeaORM 连接池
// - 定义 transfer_job / transfer_item / transfer_result_message / file_cache 实体
// - 提供基础迁移与插入测试
#[cfg(test)]
use sea_orm::ConnectionTrait;
use sea_orm::Database;
use std::path::{Path, PathBuf};
use std::time::Duration;

// 正常运行的默认 SQLite 数据库。
// 启动读取配置后会调用 `init_database_url` 覆盖为 config.json 的 `storage.database_url`。
#[cfg(not(test))]
const DATABASE_URL: &str = "sqlite://db.sqlite?mode=rwc";

// 测试会执行 migration down，必须和正常运行库隔离。
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

/// 测试库结构自检。
///
/// SeaORM migration 会记录“迁移已执行”，当本地残留旧版 `db.test.sqlite` 时，
/// 修改过的第一版迁移不会自动重建表。测试环境可以安全重建数据库，正式运行仍只依赖 migration。
#[cfg(test)]
pub(crate) async fn ensure_test_schema_current(
    db: &sea_orm::DatabaseConnection,
) -> anyhow::Result<()> {
    use migration::MigratorTrait;

    migration::Migrator::up(db, None).await?;
    if test_schema_has_required_columns(db).await? {
        return Ok(());
    }

    tracing::warn!("test database schema is stale, rebuilding test schema");
    migration::Migrator::down(db, None).await?;
    migration::Migrator::up(db, None).await?;
    Ok(())
}

/// 检查本轮测试依赖的关键列是否存在。
#[cfg(test)]
async fn test_schema_has_required_columns(
    db: &sea_orm::DatabaseConnection,
) -> anyhow::Result<bool> {
    Ok(
        sqlite_table_has_column(db, "transfer_job", "source_kind").await?
            && sqlite_table_has_column(db, "transfer_item", "file_owner_client_role").await?
            && sqlite_table_has_column(db, "transfer_result_message", "message_link").await?
            && sqlite_table_has_column(db, "file_cache", "owner_client_role").await?,
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

// DB 测试共用同一个 SQLite 文件与全局连接池。
// 为避免 migration up/down 与插入测试并发互相影响，这里串行执行。
#[cfg(test)]
pub(crate) static TEST_DB_LOCK: std::sync::LazyLock<tokio::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| tokio::sync::Mutex::new(()));

pub(crate) mod file_cache;
pub(crate) mod transfer_item;
pub(crate) mod transfer_job;
pub(crate) mod transfer_result_message;

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
