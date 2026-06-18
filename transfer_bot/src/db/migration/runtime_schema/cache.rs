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
            .table("file_cache")
            .if_not_exists()
            .col(
                ColumnDef::new("owner_client_role")
                    .string()
                    .not_null()
                    .default("user"),
            )
            .col(ColumnDef::new("file_key").string().not_null())
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
            .primary_key(Index::create().col("owner_client_role").col("file_key"))
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("file_cache_status_last_used_idx")
            .table("file_cache")
            .col("status")
            .col("last_used_at")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("file_cache_gc_due_idx")
            .table("file_cache")
            .col("active_refs")
            .col("delete_after")
            .to_owned(),
    )
    .await?;

    Ok(())
}

pub(super) async fn drop<C>(db: &C) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    exec_schema_statement(db, Table::drop().table("file_cache").if_exists().to_owned()).await?;
    Ok(())
}
