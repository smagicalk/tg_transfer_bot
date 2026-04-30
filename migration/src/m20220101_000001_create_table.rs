// 初始化表结构迁移：
// - transfer_job   任务主表
// - transfer_item  任务子项表
// - file_cache     文件缓存表
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // 创建表与索引。
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // transfer_job：一条 /transfer 请求对应一条主任务。
        manager
            .create_table(
                Table::create()
                    .table("transfer_job")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("request_chat_id").big_integer().not_null())
                    .col(
                        ColumnDef::new("request_message_id")
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new("source_link").string().not_null())
                    .col(ColumnDef::new("source_chat_id").big_integer().not_null())
                    .col(ColumnDef::new("source_message_id").big_integer().not_null())
                    .col(
                        ColumnDef::new("source_album_id")
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new("target_chat_id").big_integer().not_null())
                    .col(ColumnDef::new("result_message_id").big_integer())
                    .col(ColumnDef::new("result_message_link").string())
                    .col(ColumnDef::new("status").string().not_null())
                    .col(
                        ColumnDef::new("total_items")
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new("done_items").integer().not_null().default(0))
                    .col(
                        ColumnDef::new("failed_items")
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new("retry_count")
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new("last_error").string())
                    .col(
                        ColumnDef::new("created_at")
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new("updated_at")
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new("finished_at").timestamp_with_time_zone())
                    // 请求消息幂等：同一 request_chat_id + request_message_id 只允许一条任务。
                    .index(
                        sea_query::Index::create()
                            .name("transfer_job_request_uk")
                            .col("request_chat_id")
                            .col("request_message_id")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // 任务恢复 / 调度扫描索引。
        manager
            .create_index(
                sea_query::Index::create()
                    .if_not_exists()
                    .name("transfer_job_status_updated_idx")
                    .table("transfer_job")
                    .col("status")
                    .col("updated_at")
                    .to_owned(),
            )
            .await?;

        // 按 source_link 快速检索历史任务。
        manager
            .create_index(
                sea_query::Index::create()
                    .if_not_exists()
                    .name("transfer_job_source_link_idx")
                    .table("transfer_job")
                    .col("source_link")
                    .to_owned(),
            )
            .await?;

        // 按 source_link + target_chat_id + status 命中历史任务，用于去重转存。
        manager
            .create_index(
                sea_query::Index::create()
                    .if_not_exists()
                    .name("transfer_job_source_target_status_idx")
                    .table("transfer_job")
                    .col("source_link")
                    .col("target_chat_id")
                    .col("status")
                    .to_owned(),
            )
            .await?;

        // transfer_item：主任务下的消息级执行记录。
        manager
            .create_table(
                Table::create()
                    .table("transfer_item")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("job_id").big_integer().not_null())
                    .col(ColumnDef::new("source_chat_id").big_integer().not_null())
                    .col(ColumnDef::new("source_message_id").big_integer().not_null())
                    .col(ColumnDef::new("file_key").string().not_null())
                    .col(ColumnDef::new("status").string().not_null())
                    .col(
                        ColumnDef::new("retry_count")
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new("error_message").string())
                    .col(
                        ColumnDef::new("file_ref_released")
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new("created_at")
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new("updated_at")
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    // 主任务删除时级联删除子项。
                    .foreign_key(
                        sea_query::ForeignKeyCreateStatement::new()
                            .name("transfer_item_job_fk")
                            .from_tbl("transfer_item")
                            .from_col("job_id")
                            .to_tbl("transfer_job")
                            .to_col("id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // 任务内去重：同任务下同一 source_chat_id + source_message_id 唯一。
                    .index(
                        sea_query::Index::create()
                            .name("transfer_item_job_source_uk")
                            .col("job_id")
                            .col("source_chat_id")
                            .col("source_message_id")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // 任务子项轮询索引。
        manager
            .create_index(
                sea_query::Index::create()
                    .if_not_exists()
                    .name("transfer_item_job_status_idx")
                    .table("transfer_item")
                    .col("job_id")
                    .col("status")
                    .to_owned(),
            )
            .await?;

        // file_cache：按 file_key 跨任务下载去重。
        manager
            .create_table(
                Table::create()
                    .table("file_cache")
                    .if_not_exists()
                    .col(ColumnDef::new("file_key").string().not_null().primary_key())
                    .col(ColumnDef::new("status").string().not_null())
                    .col(ColumnDef::new("size_bytes").big_integer())
                    .col(ColumnDef::new("td_file_id").integer())
                    .col(ColumnDef::new("local_path").string())
                    .col(ColumnDef::new("last_error").string())
                    .col(
                        ColumnDef::new("active_refs")
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new("last_ref_zero_at").timestamp_with_time_zone())
                    .col(ColumnDef::new("delete_after").timestamp_with_time_zone())
                    .col(
                        ColumnDef::new("created_at")
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new("updated_at")
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new("last_used_at")
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 缓存清理索引：按状态与最近使用时间回收。
        manager
            .create_index(
                sea_query::Index::create()
                    .if_not_exists()
                    .name("file_cache_status_last_used_idx")
                    .table("file_cache")
                    .col("status")
                    .col("last_used_at")
                    .to_owned(),
            )
            .await?;

        // 删除队列扫描索引：按引用计数与到期时间取待删文件。
        manager
            .create_index(
                sea_query::Index::create()
                    .if_not_exists()
                    .name("file_cache_gc_due_idx")
                    .table("file_cache")
                    .col("active_refs")
                    .col("delete_after")
                    .to_owned(),
            )
            .await
    }

    // 回滚：按依赖反序删除。
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                sea_query::Index::drop()
                    .table("file_cache")
                    .name("file_cache_gc_due_idx")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                sea_query::Index::drop()
                    .table("file_cache")
                    .name("file_cache_status_last_used_idx")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table("file_cache").if_exists().to_owned())
            .await?;

        manager
            .drop_index(
                sea_query::Index::drop()
                    .table("transfer_item")
                    .name("transfer_item_job_status_idx")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table("transfer_item").if_exists().to_owned())
            .await?;

        manager
            .drop_index(
                sea_query::Index::drop()
                    .table("transfer_job")
                    .name("transfer_job_source_target_status_idx")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                sea_query::Index::drop()
                    .table("transfer_job")
                    .name("transfer_job_source_link_idx")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                sea_query::Index::drop()
                    .table("transfer_job")
                    .name("transfer_job_status_updated_idx")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table("transfer_job").if_exists().to_owned())
            .await
    }
}
