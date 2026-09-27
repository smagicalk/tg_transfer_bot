//! 数据库版本迁移（Migration）注册与调度模块。
//!
//! - 运行时程序启动会自动按序执行所有未完成的 Migration 脚本；
//! - 后续表结构演化只需在此注册新的 `mYYYYMMDD_NNNNNN_xxx` 迁移模块。

use sea_orm_migration::prelude::*;

mod m20260616_000001_initial_schema;
mod m20260616_000002_add_transfer_job_success_lookup_idx;
mod m20260719_000003_create_authorized_user;
mod m20260720_000004_add_authorized_user_profile;
pub(crate) mod runtime_schema;

/// SeaORM 数据库迁移执行器。
pub(crate) struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    /// 注册全部版本迁移脚本序列。
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260616_000001_initial_schema::Migration),
            Box::new(m20260616_000002_add_transfer_job_success_lookup_idx::Migration),
            Box::new(m20260719_000003_create_authorized_user::Migration),
            Box::new(m20260720_000004_add_authorized_user_profile::Migration),
        ]
    }
}
