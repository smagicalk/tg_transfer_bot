use sea_orm::ConnectionTrait;
use sea_orm::sea_query::{ColumnDef, Index, Table};

use super::exec_schema_statement;

pub(super) async fn create<C>(db: &C) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    exec_schema_statement(
        db,
        Table::create()
            .table("menu_input_draft")
            .if_not_exists()
            .col(ColumnDef::new("request_chat_id").big_integer().not_null())
            .col(ColumnDef::new("sender_user_id").big_integer().not_null())
            .col(ColumnDef::new("step").string().not_null())
            .col(ColumnDef::new("input_kind").string())
            .col(ColumnDef::new("job_action").string())
            .col(ColumnDef::new("source_link").string())
            .col(ColumnDef::new("target_chat_id").big_integer())
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
                ColumnDef::new("expires_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .primary_key(Index::create().col("request_chat_id").col("sender_user_id"))
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("menu_input_draft_expires_idx")
            .table("menu_input_draft")
            .col("expires_at")
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
            .table("menu_input_draft")
            .if_exists()
            .to_owned(),
    )
    .await?;
    Ok(())
}
