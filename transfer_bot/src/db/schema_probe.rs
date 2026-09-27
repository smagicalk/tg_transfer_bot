//! 测试环境数据库结构自检与重建模块。
//!
//! # 核心职责
//! 1. **结构完整性探测**：在单元测试或集成测试执行前，探测测试数据库是否缺少当前代码依赖的关键数据表与字段。
//! 2. **自动重建机制**：若发现现有测试数据库存在旧版本遗留或缺失列，自动进行 drop + create 全量重建，防止开发期由于 schema 频繁变更污染测试状态。
//! 3. **跨方言元数据兼容**：针对 SQLite（`PRAGMA table_info`）与 PostgreSQL（`information_schema.columns`）提供方言适配的元数据查询抽象。

#[cfg(test)]
use super::connection::DbDialect;
#[cfg(test)]
use sea_orm::ConnectionTrait;
#[cfg(test)]
use sea_orm::{DatabaseBackend, Value};

/// 确保测试数据库 Schema 为最新版本。
///
/// 测试环境允许直接清空并重建业务库。当发现旧测试库缺少当前代码依赖的列时，
/// 立即执行先删表再建表流程（`drop_runtime_schema` + `create_runtime_schema`），
/// 保证所有测试用例均运行在最新定义的 Schema 结构上。
///
/// # 参数
/// * `db` - 数据库连接引用
///
/// # 返回
/// 成功返回 `Ok(())`，失败返回相应错误
#[cfg(test)]
pub(crate) async fn ensure_test_schema_current(
    db: &sea_orm::DatabaseConnection,
) -> anyhow::Result<()> {
    // 首次确保基础表存在
    crate::db::migration::runtime_schema::create_runtime_schema(db).await?;
    // 校验所有依赖的关键字段是否齐全
    if test_schema_has_required_columns(db).await? {
        return Ok(());
    }

    // 字段缺失，打印警告并全量重建
    tracing::warn!("test database schema is stale, rebuilding test schema");
    rebuild_test_schema(db).await?;
    Ok(())
}

/// 测试环境直接删表并全量重建。
///
/// # 参数
/// * `db` - 数据库连接引用
///
/// # 返回
/// 成功返回 `Ok(())`，建表失败返回错误
#[cfg(test)]
pub(crate) async fn rebuild_test_schema(db: &sea_orm::DatabaseConnection) -> anyhow::Result<()> {
    crate::db::migration::runtime_schema::drop_runtime_schema(db).await?;
    crate::db::migration::runtime_schema::create_runtime_schema(db).await
}

/// 将 SeaORM 的 `DatabaseBackend` 映射到当前项目显式支持的数据库方言 `DbDialect`。
///
/// # 参数
/// * `backend` - SeaORM 后端枚举
///
/// # 返回
/// 支持则返回 `Some(DbDialect)`，否则返回 `None`
#[cfg(test)]
fn backend_dialect(backend: DatabaseBackend) -> Option<DbDialect> {
    match backend {
        DatabaseBackend::Sqlite => Some(DbDialect::Sqlite),
        DatabaseBackend::Postgres => Some(DbDialect::Postgres),
        _ => None,
    }
}

/// 为测试元数据探测生成匹配底层数据库方言的 `sea_orm::Statement`。
///
/// SQLite 与 PostgreSQL 的占位符语法不同（`?` 与 `$1, $2`），因此不能直接复用同一段 SQL 文本。
///
/// # 参数
/// * `backend` - 数据库后端类型
/// * `sqlite_sql` - 针对 SQLite 方言的 SQL 语句
/// * `postgres_sql` - 针对 PostgreSQL 方言的 SQL 语句
/// * `values` - SQL 参数绑定值列表
///
/// # 返回
/// 成功返回组装好的 `Statement`，若后端不支持则报错
#[cfg(test)]
pub(crate) fn raw_statement_for_backend(
    backend: DatabaseBackend,
    sqlite_sql: &str,
    postgres_sql: &str,
    values: Vec<Value>,
) -> anyhow::Result<sea_orm::Statement> {
    let sql = match backend_dialect(backend) {
        Some(DbDialect::Sqlite) => sqlite_sql,
        Some(DbDialect::Postgres) => postgres_sql,
        None => anyhow::bail!("unsupported database backend for raw sql: {backend:?}"),
    };
    Ok(sea_orm::Statement::from_sql_and_values(
        backend, sql, values,
    ))
}

/// 检查测试数据库中是否包含当前版本代码所依赖的全部核心列。
///
/// # 参数
/// * `db` - 数据库连接引用
///
/// # 返回
/// 若所有核心列均存在则返回 `Ok(true)`，只要有一列缺失则返回 `Ok(false)`
#[cfg(test)]
pub(crate) async fn test_schema_has_required_columns(
    db: &sea_orm::DatabaseConnection,
) -> anyhow::Result<bool> {
    Ok(
        // 转存任务表核心字段
        test_table_has_column(db, "transfer_job", "source_kind").await?
            && test_table_has_column(db, "transfer_job", "owner_user_id").await?
            && test_table_has_column(db, "transfer_job", "allow_user_fallback").await?
            // 转存明细项表：文件所属客户端角色
            && test_table_has_column(db, "transfer_item", "file_owner_client_role").await?
            // 转存结果消息关联表：消息链接
            && test_table_has_column(db, "transfer_result_message", "message_link").await?
            // 文件缓存表：缓存归属客户端角色
            && test_table_has_column(db, "file_cache", "owner_client_role").await?
            // 菜单输入草稿表：草稿过期时间
            && test_table_has_column(db, "menu_input_draft", "expires_at").await?
            // 运行时配置表：菜单输入超时秒数
            && test_table_has_column(db, "transfer_runtime_config", "menu_input_timeout_seconds")
                .await?
            // 动态授权用户表：用户标识与用户名称快照
            && test_table_has_column(db, "authorized_user", "user_id").await?
            && test_table_has_column(db, "authorized_user", "display_name").await?
            && test_table_has_column(db, "authorized_user", "username").await?,
    )
}

/// 按底层数据库后端检查指定表的指定列是否存在。
///
/// SeaORM/SeaQuery 负责通用建表，但“读取数据库元数据”没有统一抽象；
/// 这里仅在测试自检中保留最小方言分支，业务读写路径不依赖手写 SQL。
///
/// # 参数
/// * `db` - 数据库连接引用
/// * `table` - 表名
/// * `column` - 列名
///
/// # 返回
/// 存在返回 `Ok(true)`，不存在返回 `Ok(false)`，遇到未支持方言报错
#[cfg(test)]
async fn test_table_has_column(
    db: &sea_orm::DatabaseConnection,
    table: &str,
    column: &str,
) -> anyhow::Result<bool> {
    match db.get_database_backend() {
        DatabaseBackend::Sqlite => {
            // SQLite 使用 PRAGMA table_info 查询字段列表
            let sql = format!("PRAGMA table_info({table})");
            let statement = sea_orm::Statement::from_string(DatabaseBackend::Sqlite, sql);
            let rows = db.query_all_raw(statement).await?;
            for row in rows {
                let name: String = row.try_get("", "name")?;
                if name == column {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        DatabaseBackend::Postgres => {
            // PostgreSQL 查询 information_schema.columns
            let statement = raw_statement_for_backend(
                DatabaseBackend::Postgres,
                "",
                r#"
                SELECT 1
                FROM information_schema.columns
                WHERE table_schema = current_schema()
                  AND table_name = $1
                  AND column_name = $2
                LIMIT 1
                "#,
                vec![table.into(), column.into()],
            )?;
            Ok(!db.query_all_raw(statement).await?.is_empty())
        }
        backend => anyhow::bail!("unsupported database backend for test schema probe: {backend:?}"),
    }
}

