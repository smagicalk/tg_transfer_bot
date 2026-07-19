// 第二个增量 migration：
// 为“按 source_link + target_chat_id 复用最近成功转存结果”补专用索引。
// 这条查询在重复转存与 /lookup 命中成功任务时都会走到，按 finished_at 倒序取最近一条。

use sea_orm_migration::prelude::*;

const INDEX_NAME: &str = "transfer_job_success_lookup_idx";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
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
