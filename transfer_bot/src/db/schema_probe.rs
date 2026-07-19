// 测试 schema 自检：
// - 检查测试库是否缺少当前代码依赖的关键列
// - 若发现旧测试库残留，则重建测试 schema

#[cfg(test)]
use super::connection::DbDialect;
#[cfg(test)]
use sea_orm::ConnectionTrait;
#[cfg(test)]
use sea_orm::{DatabaseBackend, Value};

/// 测试库结构自检。
///
/// 测试环境允许直接重建业务库；当发现旧测试库缺少当前代码依赖的列时，
/// 直接 drop + create，避免开发期 schema 演进把测试状态拖脏。
#[cfg(test)]
pub(crate) async fn ensure_test_schema_current(
    db: &sea_orm::DatabaseConnection,
) -> anyhow::Result<()> {
    crate::db::migration::runtime_schema::create_runtime_schema(db).await?;
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
    crate::db::migration::runtime_schema::drop_runtime_schema(db).await?;
    crate::db::migration::runtime_schema::create_runtime_schema(db).await
}

/// 将 SeaORM backend 映射到当前项目显式支持的数据库方言。
#[cfg(test)]
fn backend_dialect(backend: DatabaseBackend) -> Option<DbDialect> {
    match backend {
        DatabaseBackend::Sqlite => Some(DbDialect::Sqlite),
        DatabaseBackend::Postgres => Some(DbDialect::Postgres),
        _ => None,
    }
}

/// 为测试元数据探测生成匹配方言的 `Statement`。
///
/// SQLite 与 PostgreSQL 的占位符语法不同，因此不能直接复用同一段 SQL 文本。
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

/// 检查本轮测试依赖的关键列是否存在。
#[cfg(test)]
pub(crate) async fn test_schema_has_required_columns(
    db: &sea_orm::DatabaseConnection,
) -> anyhow::Result<bool> {
    Ok(
        test_table_has_column(db, "transfer_job", "source_kind").await?
            && test_table_has_column(db, "transfer_job", "owner_user_id").await?
            && test_table_has_column(db, "transfer_job", "allow_user_fallback").await?
            && test_table_has_column(db, "transfer_item", "file_owner_client_role").await?
            && test_table_has_column(db, "transfer_result_message", "message_link").await?
            && test_table_has_column(db, "file_cache", "owner_client_role").await?
            && test_table_has_column(db, "menu_input_draft", "expires_at").await?
            && test_table_has_column(db, "transfer_runtime_config", "menu_input_timeout_seconds")
                .await?,
    )
}

/// 测试环境按数据库后端检查指定列是否已存在。
///
/// SeaORM/SeaQuery 负责通用建表，但“读取数据库元数据”没有统一抽象；
/// 这里仅在测试自检中保留最小方言分支，业务读写路径不依赖手写 SQL。
#[cfg(test)]
async fn test_table_has_column(
    db: &sea_orm::DatabaseConnection,
    table: &str,
    column: &str,
) -> anyhow::Result<bool> {
    match db.get_database_backend() {
        DatabaseBackend::Sqlite => {
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
