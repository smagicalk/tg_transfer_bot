//! 动态授权用户白名单表（`authorized_user`）DDL 定义与迁移。

use sea_orm::ConnectionTrait;
use sea_orm::sea_query::{ColumnDef, Table};

use super::exec_schema_statement;

/// 创建 `authorized_user` 数据表结构（若不存在）。
pub(super) async fn create<C>(db: &C) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    exec_schema_statement(
        db,
        Table::create()
            .table("authorized_user")
            .if_not_exists()
            .col(
                ColumnDef::new("user_id")
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new("display_name").string())
            .col(ColumnDef::new("username").string())
            .col(
                ColumnDef::new("created_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .to_owned(),
    )
    .await
}

/// 删除 `authorized_user` 数据表结构（若存在）。
pub(super) async fn drop<C>(db: &C) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    exec_schema_statement(
        db,
        Table::drop()
            .table("authorized_user")
            .if_exists()
            .to_owned(),
    )
    .await
}
