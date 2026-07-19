// 数据库连接初始化：
// - 解析业务数据库连接串
// - 初始化 SeaORM 连接池
// - 处理 SQLite 文件库目录创建

use sea_orm::Database;
use std::path::{Path, PathBuf};
use std::time::Duration;

// 正常运行的默认业务数据库；配置文件可切换为 SQLite 或 PostgreSQL。
// 启动读取配置后会调用 `init_database_url` 覆盖为 config.json 的 `storage.database_url`。
#[cfg(not(test))]
const DATABASE_URL: &str = "sqlite://tg/app/transfer.sqlite?mode=rwc";

// 测试库放到 `target/` 下，避免污染源码目录，也规避旧测试库残留导致的只读/锁文件问题。
#[cfg(test)]
const DATABASE_URL: &str = "sqlite://target/test-data/db.test.sqlite?mode=rwc";

// 运行期数据库连接串。
// 必须在第一次 `get_db()` 之前设置；测试环境保持固定测试库，避免被运行配置污染。
#[cfg(not(test))]
static DATABASE_URL_OVERRIDE: std::sync::OnceLock<String> = std::sync::OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DbDialect {
    Sqlite,
    Postgres,
}

/// 初始化业务数据库连接串。
///
/// 业务数据库保存转存任务、子项和文件引用，不是 TDLib 账号数据库。
#[cfg(not(test))]
pub(crate) async fn init_database_url(database_url: impl Into<String>) -> anyhow::Result<()> {
    let database_url = database_url.into();
    if super::DB_POOL.initialized() {
        anyhow::bail!("database pool already initialized before database url was configured");
    }
    prepare_database_parent_dir(&database_url).await?;
    DATABASE_URL_OVERRIDE
        .set(database_url)
        .map_err(|_| anyhow::anyhow!("database url already initialized"))?;
    Ok(())
}

/// 测试环境固定使用 `db.test.sqlite`，不允许运行配置改写。
#[cfg(test)]
pub(crate) async fn init_database_url(_database_url: impl Into<String>) -> anyhow::Result<()> {
    if super::DB_POOL.initialized() {
        anyhow::bail!("test database pool already initialized");
    }
    Ok(())
}

/// 初始化业务数据库连接。
pub(crate) async fn init_db() -> anyhow::Result<sea_orm::DatabaseConnection> {
    let database_url = runtime_database_url();
    prepare_database_parent_dir(database_url).await?;
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
pub(crate) fn runtime_database_url() -> &'static str {
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

/// 仅 SQLite 文件库需要先创建父目录；PG 等服务型数据库直接跳过。
async fn prepare_database_parent_dir(database_url: &str) -> anyhow::Result<()> {
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
pub(crate) fn sqlite_file_path(database_url: &str) -> Option<PathBuf> {
    if database_dialect(database_url) != Some(DbDialect::Sqlite) {
        return None;
    }
    let path = database_url
        .strip_prefix("sqlite://")
        .or_else(|| database_url.strip_prefix("sqlite:"))?;
    let path = path.split('?').next().unwrap_or(path);
    if path.is_empty() || path == ":memory:" {
        return None;
    }
    Some(Path::new(path).to_path_buf())
}

pub(crate) fn database_dialect(database_url: &str) -> Option<DbDialect> {
    if database_url.starts_with("sqlite://") || database_url.starts_with("sqlite:") {
        return Some(DbDialect::Sqlite);
    }
    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        return Some(DbDialect::Postgres);
    }
    None
}
