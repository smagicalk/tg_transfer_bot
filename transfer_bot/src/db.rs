//! 业务数据持久化层核心模块。
//!
//! 负责：
//! - 初始化并持有全局 SeaORM 数据库连接池（当前默认适配 SQLite，预留 PostgreSQL 兼容）；
//! - 启动时自动执行 SeaORM Migration 迁移确保各业务表结构完整；
//! - 汇聚转存任务、子项、文件缓存引用计数、菜单输入草稿、运行时配置及授权白名单等数据表实体。

use sea_orm_migration::MigratorTrait;

mod connection;
pub(crate) mod migration;
mod schema_probe;

/// 全局数据库连接池句柄（惰性一次性初始化）。
pub(crate) static DB_POOL: tokio::sync::OnceCell<sea_orm::DatabaseConnection> =
    tokio::sync::OnceCell::const_new();
pub(crate) use connection::init_database_url;
#[cfg(test)]
pub(crate) use schema_probe::{
    ensure_test_schema_current, raw_statement_for_backend, rebuild_test_schema,
    test_schema_has_required_columns,
};

/// 获取全局数据库连接池引用（必要时自动触发初始化连接流程）。
pub(crate) async fn get_db<'db>() -> anyhow::Result<&'db sea_orm::DatabaseConnection> {
    DB_POOL.get_or_try_init(connection::init_db).await
}

/// 启动时确保业务表结构存在，并执行所有未完成的 Migration 迁移。
///
/// 正常运行路径只执行 migration；`create_runtime_schema` 保留给初始迁移和测试重建复用。
pub(crate) async fn ensure_runtime_schema(db: &sea_orm::DatabaseConnection) -> anyhow::Result<()> {
    crate::db::migration::Migrator::up(db, None).await?;
    Ok(())
}

/// 测试库结构自检互斥锁。
///
/// 测试环境允许直接重建业务库；当发现旧测试库缺少当前代码依赖的列时，
/// 直接 drop + create，避免开发期 schema 演进把测试状态拖脏。
/// DB 测试共用同一个 SQLite 文件与全局连接池，为避免重建表结构与插入测试并发互相影响，这里串行执行。
#[cfg(test)]
pub(crate) static TEST_DB_LOCK: std::sync::LazyLock<tokio::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| tokio::sync::Mutex::new(()));

/// 动态授权用户白名单表实体模型。
pub(crate) mod authorized_user;
/// 本地文件缓存元数据与引用计数表实体模型。
pub(crate) mod file_cache;
/// 菜单交互分步草稿状态表实体模型。
pub(crate) mod menu_input_draft;
/// 转存任务关联的子文件条目实体模型。
pub(crate) mod transfer_item;
/// 转存主任务执行状态与元数据实体模型。
pub(crate) mod transfer_job;
/// 转存完成发送的目标卡片消息定位表实体模型。
pub(crate) mod transfer_result_message;
/// 转存调度与 GC 运行时控制配置表实体模型。
pub(crate) mod transfer_runtime_config;
/// 目标频道快捷别名映射表实体模型。
pub(crate) mod transfer_target_alias;
/// 默认目标频道与规则配置表实体模型。
pub(crate) mod transfer_target_config;

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
