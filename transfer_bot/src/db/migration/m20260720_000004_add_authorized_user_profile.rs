// 为动态授权用户保存名称快照，便于授权管理界面识别用户。

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite 只允许单个 ALTER 选项，因此两个新增列分别执行。
        if !manager
            .has_column("authorized_user", "display_name")
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table("authorized_user")
                        .add_column(ColumnDef::new("display_name").string())
                        .to_owned(),
                )
                .await?;
        }
        if !manager.has_column("authorized_user", "username").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table("authorized_user")
                        .add_column(ColumnDef::new("username").string())
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_column("authorized_user", "username").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table("authorized_user")
                        .drop_column("username")
                        .to_owned(),
                )
                .await?;
        }
        if manager
            .has_column("authorized_user", "display_name")
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table("authorized_user")
                        .drop_column("display_name")
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::Database;

    use super::*;

    #[tokio::test]
    async fn test_profile_migration_upgrades_old_sqlite_table_idempotently() -> anyhow::Result<()> {
        let db = Database::connect("sqlite::memory:").await?;
        let manager = SchemaManager::new(&db);
        manager
            .create_table(
                Table::create()
                    .table("authorized_user")
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
            .await?;

        let migration = Migration;
        migration.up(&manager).await?;
        migration.up(&manager).await?;

        assert!(
            manager
                .has_column("authorized_user", "display_name")
                .await?
        );
        assert!(manager.has_column("authorized_user", "username").await?);
        Ok(())
    }

    #[tokio::test]
    async fn test_profile_migration_follows_legacy_authorized_user_migration() -> anyhow::Result<()>
    {
        let db = Database::connect("sqlite::memory:").await?;
        let manager = SchemaManager::new(&db);
        crate::db::migration::m20260719_000003_create_authorized_user::Migration
            .up(&manager)
            .await?;

        assert!(
            !manager
                .has_column("authorized_user", "display_name")
                .await?
        );
        assert!(!manager.has_column("authorized_user", "username").await?);

        Migration.up(&manager).await?;
        assert!(
            manager
                .has_column("authorized_user", "display_name")
                .await?
        );
        assert!(manager.has_column("authorized_user", "username").await?);
        Ok(())
    }

    #[tokio::test]
    async fn test_full_migration_chain_accepts_current_schema_columns() -> anyhow::Result<()> {
        let db = Database::connect("sqlite::memory:").await?;
        crate::db::migration::Migrator::up(&db, None).await?;
        let manager = SchemaManager::new(&db);

        assert!(
            manager
                .has_column("authorized_user", "display_name")
                .await?
        );
        assert!(manager.has_column("authorized_user", "username").await?);
        Ok(())
    }
}
