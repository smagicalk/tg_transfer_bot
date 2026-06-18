use sea_orm::ConnectionTrait;
use sea_orm::sea_query::{ColumnDef, ForeignKeyAction, ForeignKeyCreateStatement, Index, Table};

use super::exec_schema_statement;

pub(super) async fn create<C>(db: &C) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    exec_schema_statement(
        db,
        Table::create()
            .table("user_account")
            .if_not_exists()
            .col(
                ColumnDef::new("telegram_user_id")
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new("role").string().not_null().default("user"))
            .col(
                ColumnDef::new("points_balance")
                    .big_integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new("total_points_added")
                    .big_integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new("total_points_spent")
                    .big_integer()
                    .not_null()
                    .default(0),
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
        Index::create()
            .if_not_exists()
            .name("user_account_role_idx")
            .table("user_account")
            .col("role")
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Table::create()
            .table("point_ledger")
            .if_not_exists()
            .col(
                ColumnDef::new("id")
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new("telegram_user_id").big_integer().not_null())
            .col(ColumnDef::new("delta").big_integer().not_null())
            .col(ColumnDef::new("balance_after").big_integer().not_null())
            .col(ColumnDef::new("reason").string().not_null())
            .col(ColumnDef::new("job_id").big_integer())
            .col(ColumnDef::new("request_chat_id").big_integer())
            .col(ColumnDef::new("request_message_id").big_integer())
            .col(ColumnDef::new("idempotency_key").string())
            .col(ColumnDef::new("created_by").big_integer())
            .col(
                ColumnDef::new("created_at")
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("point_ledger_account_fk")
                    .from_tbl("point_ledger")
                    .from_col("telegram_user_id")
                    .to_tbl("user_account")
                    .to_col("telegram_user_id")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("point_ledger_idempotency_uk")
                    .col("idempotency_key")
                    .unique(),
            )
            .to_owned(),
    )
    .await?;

    exec_schema_statement(
        db,
        Index::create()
            .if_not_exists()
            .name("point_ledger_user_created_idx")
            .table("point_ledger")
            .col("telegram_user_id")
            .col("created_at")
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
        Table::drop().table("point_ledger").if_exists().to_owned(),
    )
    .await?;
    exec_schema_statement(
        db,
        Table::drop().table("user_account").if_exists().to_owned(),
    )
    .await?;
    Ok(())
}
