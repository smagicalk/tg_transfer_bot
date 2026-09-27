//! 动态授权用户白名单表（Authorized User）增量迁移定义。
//!
//! # 迁移说明
//! - 创建 `authorized_user` 基础表结构。
//! - 历史阶段该表仅包含 `user_id` 主键与 `created_at` 授权时间戳；后续用户名与昵称扩展字段由迁移 `000004` 增量引入。
//! - 此处保持原始建表 DDL 定义，不直接调用最新的 runtime schema helper，避免把未来版本字段提前带入历史迁移快照。

use sea_orm_migration::prelude::*;

/// 授权用户表创建迁移对象。
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// 执行建表迁移（Up）。
    ///
    /// 创建 `authorized_user` 表：
    /// - `user_id`：Telegram 用户 64 位整型 ID（主键）
    /// - `created_at`：带时区的记录创建/授权授予时间戳
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

    /// 执行回滚迁移（Down）。
    ///
    /// 删除 `authorized_user` 表。
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::db::migration::runtime_schema::drop_access_schema(manager.get_connection())
            .await
            .map_err(|err| DbErr::Migration(err.to_string()))
    }
}

