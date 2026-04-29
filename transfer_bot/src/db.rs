// 数据库模块：
// - 初始化 SeaORM 连接池
// - 定义 transfer_job / transfer_item / file_cache 实体
// - 提供基础迁移与插入测试
use sea_orm::Database;
use std::time::Duration;

// 正常运行使用的 SQLite 数据库。
#[cfg(not(test))]
const DATABASE_URL: &str = "sqlite://db.sqlite?mode=rwc";

// 测试会执行 migration down，必须和正常运行库隔离。
#[cfg(test)]
const DATABASE_URL: &str = "sqlite://db.test.sqlite?mode=rwc";

// 全局数据库连接池句柄（惰性初始化）。
pub(crate) static DB_POOL: tokio::sync::OnceCell<sea_orm::DatabaseConnection> =
    tokio::sync::OnceCell::const_new();

// 初始化数据库连接（SQLite）。
pub(crate) async fn init_db() -> anyhow::Result<sea_orm::DatabaseConnection> {
    let mut opt = sea_orm::ConnectOptions::new(DATABASE_URL);
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

// 获取数据库连接（必要时自动初始化）。
pub(crate) async fn get_db<'db>() -> anyhow::Result<&'db sea_orm::DatabaseConnection> {
    DB_POOL.get_or_try_init(init_db).await
}

// DB 测试共用同一个 SQLite 文件与全局连接池。
// 为避免 migration up/down 与插入测试并发互相影响，这里串行执行。
#[cfg(test)]
pub(crate) static TEST_DB_LOCK: std::sync::LazyLock<tokio::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| tokio::sync::Mutex::new(()));

pub(crate) mod file_cache;
pub(crate) mod transfer_item;
pub(crate) mod transfer_job;

#[cfg(test)]
mod tests;
