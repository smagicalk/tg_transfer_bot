// 初始 schema 迁移：
// 当前项目还处于开发期，首次引入 migration 时直接接管现有完整 schema。
// 迁移内容复用 db/migration/runtime_schema.rs 的 schema helper，避免 migration 文件和实体定义长期复制分叉。

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::db::migration::runtime_schema::create_runtime_schema(manager.get_connection())
            .await
            .map_err(|err| DbErr::Migration(err.to_string()))
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::db::migration::runtime_schema::drop_runtime_schema(manager.get_connection())
            .await
            .map_err(|err| DbErr::Migration(err.to_string()))
    }
}
