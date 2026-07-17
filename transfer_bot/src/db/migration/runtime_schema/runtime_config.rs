use sea_orm::ConnectionTrait;
use sea_orm::sea_query::{ColumnDef, Table};

use super::exec_schema_statement;

pub(super) async fn create<C>(db: &C) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    exec_schema_statement(
        db,
        Table::create()
            .table("transfer_runtime_config")
            .if_not_exists()
            .col(ColumnDef::new("id").integer().not_null().primary_key())
            .col(ColumnDef::new("job_concurrency").big_integer().not_null())
            .col(
                ColumnDef::new("file_delete_delay_minutes")
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new("file_gc_interval_seconds")
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new("progress_edit_interval_seconds")
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new("downloads_default_page_size")
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new("menu_input_timeout_seconds")
                    .big_integer()
                    .not_null(),
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
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Table::create()
            .table("transfer_target_config")
            .if_not_exists()
            .col(ColumnDef::new("id").integer().not_null().primary_key())
            .col(ColumnDef::new("default_chat_id").big_integer().not_null())
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
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Table::create()
            .table("transfer_target_alias")
            .if_not_exists()
            .col(ColumnDef::new("alias").string().not_null().primary_key())
            .col(ColumnDef::new("target_chat_id").big_integer().not_null())
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
            .to_owned(),
    )
    .await?;

    Ok(())
}

pub(super) async fn drop<C>(db: &C) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    exec_schema_statement(
        db,
        Table::drop()
            .table("transfer_target_alias")
            .if_exists()
            .to_owned(),
    )
    .await?;
    exec_schema_statement(
        db,
        Table::drop()
            .table("transfer_target_config")
            .if_exists()
            .to_owned(),
    )
    .await?;
    exec_schema_statement(
        db,
        Table::drop()
            .table("transfer_runtime_config")
            .if_exists()
            .to_owned(),
    )
    .await?;
    Ok(())
}
