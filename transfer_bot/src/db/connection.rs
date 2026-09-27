//! 数据库连接管理与连接池初始化。
//!
//! 负责：
//! - 解析业务数据库 URL 并识别数据库方言（SQLite / PostgreSQL）；
//! - 自动为本地 SQLite 文件数据库创建父级目录结构；
//! - 配置 SeaORM 连接池参数（连接数上限、获取超时、空闲回收等）。

use sea_orm::Database;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// 正常运行的默认业务数据库连接串；启动时可通过 `init_database_url` 由配置文件覆盖。
#[cfg(not(test))]
const DATABASE_URL: &str = "sqlite://tg/app/transfer.sqlite?mode=rwc";

/// 单元测试专用数据库路径，放置在 `target/test-data/` 下避免污染生产文件。
#[cfg(test)]
const DATABASE_URL: &str = "sqlite://target/test-data/db.test.sqlite?mode=rwc";

/// 运行期覆盖的数据库连接串全局容器（必须在首次 `get_db()` 前设置）。
#[cfg(not(test))]
static DATABASE_URL_OVERRIDE: std::sync::OnceLock<String> = std::sync::OnceLock::new();

/// 支持的底层数据库方言类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DbDialect {
    /// 本地轻量级 SQLite 文件或内存数据库。
    Sqlite,
    /// 独立部署的 PostgreSQL 数据库服务。
    Postgres,
}

/// 设置并初始化业务数据库连接串。
///
/// 业务数据库用于持久化转存任务、子项、文件引用计数与菜单输入状态。
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

/// 测试环境连接串初始化打桩实现（测试环境固定使用独立测试库）。
#[cfg(test)]
pub(crate) async fn init_database_url(_database_url: impl Into<String>) -> anyhow::Result<()> {
    if super::DB_POOL.initialized() {
        anyhow::bail!("test database pool already initialized");
    }
    Ok(())
}

/// 实际创建并配置 SeaORM 连接池实例。
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

/// 获取当前生效的业务数据库连接串。
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

/// 若连接串为 SQLite 文件库，自动检测并创建其所在的上级父目录。
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

/// 从 SQLite 连接串解析出本地目标文件路径。
///
/// 排除 `:memory:` 纯内存数据库。
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

/// 根据连接串协议前缀探测数据库方言（SQLite 或 PostgreSQL）。
pub(crate) fn database_dialect(database_url: &str) -> Option<DbDialect> {
    if database_url.starts_with("sqlite://") || database_url.starts_with("sqlite:") {
        return Some(DbDialect::Sqlite);
    }
    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        return Some(DbDialect::Postgres);
    }
    None
}
