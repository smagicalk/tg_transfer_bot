//! 转存任务历史成功记录快速查询复合索引迁移定义。
//!
//! # 迁移背景
//! 为“按 `source_link` + `target_chat_id` 查询并复用最近一次成功转存结果”补充专用复合索引。
//!
//! # 适用查询场景
//! - 用户发起重复转存时，快速检查目标频道是否已存在转存完成的记录。
//! - 执行 `/lookup` 命令快速检索历史转存结果。
//! - 查询条件：`WHERE source_link = ? AND target_chat_id = ? AND status = 'Succeeded' ORDER BY finished_at DESC LIMIT 1`。

use sea_orm_migration::prelude::*;

/// 复合索引名称常量。
const INDEX_NAME: &str = "transfer_job_success_lookup_idx";

/// 复合索引创建与删除迁移对象。
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// 执行索引创建迁移（Up）。
    ///
    /// 在 `transfer_job` 表上创建 `(source_link, target_chat_id, status, finished_at)` 四列复合索引。
    /// 包含 `if_not_exists` 保证幂等执行。
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(INDEX_NAME)
                    .table("transfer_job")
                    .col("source_link")
                    .col("target_chat_id")
                    .col("status")
                    .col("finished_at")
                    .to_owned(),
            )
            .await
    }

    /// 执行索引删除迁移（Down）。
    ///
    /// 从 `transfer_job` 表中删除 `transfer_job_success_lookup_idx` 索引。
    /// 包含 `if_exists` 保证幂等回滚。
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name(INDEX_NAME)
                    .table("transfer_job")
                    .to_owned(),
            )
            .await
    }
}
