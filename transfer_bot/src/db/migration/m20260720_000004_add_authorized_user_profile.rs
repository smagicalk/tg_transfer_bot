//! 动态授权用户快照信息（显示名称与用户名）扩展迁移定义。
//!
//! # 迁移说明
//! 为 `authorized_user` 表新增 `display_name` 与 `username` 两列，用于在后台或 Telegram 授权管理菜单中
//! 直观展示用户的昵称与用户名，而无需频繁向 Telegram 服务器请求用户资料。
//!
//! # 兼容性注意
//! - SQLite 限制单个 `ALTER TABLE` 语句中只能执行一个子操作，因此新增列和删除列均拆分为独立的 `alter_table` 语句依次执行。
//! - 使用 `has_column` 条件检查，保证多次重复执行迁移时具备幂等性。

use sea_orm_migration::prelude::*;

/// 授权用户信息扩展迁移对象。
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// 执行字段扩展迁移（Up）。
    ///
    /// 依次向 `authorized_user` 表添加：
    /// 1. `display_name`：用户展示名称（昵称）
    /// 2. `username`：Telegram 用户名（不带 @）
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

    /// 执行字段回滚迁移（Down）。
    ///
    /// 依次从 `authorized_user` 表中删除 `username` 与 `display_name` 列。
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

    /// 测试在旧版 SQLite 表结构上执行升级迁移时的幂等性。
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
        // 连续执行两次 up 验证幂等
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

    /// 测试在执行了旧迁移 000003 后紧接着执行本迁移的链式升级流程。
    #[tokio::test]
    async fn test_profile_migration_follows_legacy_authorized_user_migration() -> anyhow::Result<()>
    {
        let db = Database::connect("sqlite::memory:").await?;
        let manager = SchemaManager::new(&db);
        // 先运行 000003 迁移
        crate::db::migration::m20260719_000003_create_authorized_user::Migration
            .up(&manager)
            .await?;

        // 验证尚未存在扩展字段
        assert!(
            !manager
                .has_column("authorized_user", "display_name")
                .await?
        );
        assert!(!manager.has_column("authorized_user", "username").await?);

        // 运行本迁移
        Migration.up(&manager).await?;
        assert!(
            manager
                .has_column("authorized_user", "display_name")
                .await?
        );
        assert!(manager.has_column("authorized_user", "username").await?);
        Ok(())
    }

    /// 测试从头到尾运行 Migrator 完整迁移链后所有列均正常就绪。
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
