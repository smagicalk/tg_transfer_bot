//! 初始数据库结构（Initial Schema）迁移定义。
//!
//! # 迁移说明
//! - 首次引入 SeaORM Migration 时创建的基准迁移步骤。
//! - 直接接管项目已有的完整运行时数据表结构。
//! - 迁移实现复用 `db/migration/runtime_schema.rs` 中的建表辅助函数，
//!   避免 Migration 定义与业务实体定义（Entities）长期维护出现复制分叉。

use sea_orm_migration::prelude::*;

/// 初始 Schema 迁移对象。
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// 执行初始建表迁移（Up）。
    ///
    /// 调用 `create_runtime_schema` 依次创建项目运行所需的核心业务表与索引：
    /// - `transfer_job`（转存任务主表）
    /// - `transfer_item`（转存任务关联明细表）
    /// - `transfer_result_message`（转存结果消息关联表）
    /// - `file_cache`（下载文件与消息缓存表）
    /// - `transfer_runtime_config`（转存运行时配置表）
    /// - `transfer_target_config`（转存目标配置表）
    /// - `transfer_target_alias`（目标别名表）
    /// - `menu_input_draft`（菜单交互输入草稿表）
    /// - `authorized_user`（授权用户白名单表）
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::db::migration::runtime_schema::create_runtime_schema(manager.get_connection())
            .await
            .map_err(|err| DbErr::Migration(err.to_string()))
    }

    /// 执行回滚迁移（Down）。
    ///
    /// 调用 `drop_runtime_schema` 依次删除所有业务表及相关索引。
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::db::migration::runtime_schema::drop_runtime_schema(manager.get_connection())
            .await
            .map_err(|err| DbErr::Migration(err.to_string()))
    }
}
