// 数据库模块：
// - 初始化 SeaORM 连接池
// - 定义 transfer_job / transfer_item / transfer_result_message / file_cache / menu_input_draft / user_account / point_ledger / transfer_runtime_config 实体
// - 启动时执行 SeaORM migration；具体 DDL 统一放到 migration/runtime_schema.rs
use sea_orm_migration::MigratorTrait;

mod connection;
pub(crate) mod migration;
mod schema_probe;

// 全局数据库连接池句柄（惰性初始化）。
pub(crate) static DB_POOL: tokio::sync::OnceCell<sea_orm::DatabaseConnection> =
    tokio::sync::OnceCell::const_new();
pub(crate) use connection::init_database_url;
#[cfg(test)]
pub(crate) use schema_probe::{
    ensure_test_schema_current, raw_statement_for_backend, rebuild_test_schema,
    test_schema_has_required_columns,
};

// 获取数据库连接（必要时自动初始化）。
pub(crate) async fn get_db<'db>() -> anyhow::Result<&'db sea_orm::DatabaseConnection> {
    DB_POOL.get_or_try_init(connection::init_db).await
}

/// 启动时确保业务表结构存在。
///
/// 正常运行路径只执行 migration；`create_runtime_schema` 保留给初始迁移和测试重建复用。
pub(crate) async fn ensure_runtime_schema(db: &sea_orm::DatabaseConnection) -> anyhow::Result<()> {
    crate::db::migration::Migrator::up(db, None).await?;
    Ok(())
}

/// 测试库结构自检。
///
/// 测试环境允许直接重建业务库；当发现旧测试库缺少当前代码依赖的列时，
/// 直接 drop + create，避免开发期 schema 演进把测试状态拖脏。
// DB 测试共用同一个 SQLite 文件与全局连接池。
// 为避免重建表结构与插入测试并发互相影响，这里串行执行。
#[cfg(test)]
pub(crate) static TEST_DB_LOCK: std::sync::LazyLock<tokio::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| tokio::sync::Mutex::new(()));

pub(crate) mod access_control_admin_user;
pub(crate) mod access_control_allowed_request_chat;
pub(crate) mod access_control_allowed_target_chat;
pub(crate) mod access_control_allowed_user;
pub(crate) mod access_control_banned_user;
pub(crate) mod access_control_runtime_config;
pub(crate) mod billing_runtime_config;
pub(crate) mod file_cache;
pub(crate) mod menu_input_draft;
pub(crate) mod point_ledger;
pub(crate) mod transfer_item;
pub(crate) mod transfer_job;
pub(crate) mod transfer_result_message;
pub(crate) mod transfer_runtime_config;
pub(crate) mod transfer_target_alias;
pub(crate) mod transfer_target_config;
pub(crate) mod transfer_target_route;
pub(crate) mod user_account;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod url_tests {
    use crate::db::connection::{DbDialect, database_dialect, sqlite_file_path};
    use std::path::PathBuf;

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

    #[test]
    fn test_database_dialect_detects_postgres_url() {
        assert_eq!(
            database_dialect("postgresql://user:pass@localhost:5432/transfer"),
            Some(DbDialect::Postgres)
        );
        assert_eq!(
            database_dialect("postgres://user:pass@localhost:5432/transfer"),
            Some(DbDialect::Postgres)
        );
    }
}
