// 动态授权名单增量 migration。

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 000003 的历史版本只包含 ID 和创建时间；名称字段由 000004 增量加入。
        // 不直接复用当前 runtime schema，避免新字段被错误地记在旧 migration 中。
        manager
            .create_table(
                Table::create()
                    .table("authorized_user")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("user_id")
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new("created_at")
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::db::migration::runtime_schema::drop_access_schema(manager.get_connection())
            .await
            .map_err(|err| DbErr::Migration(err.to_string()))
    }
}
