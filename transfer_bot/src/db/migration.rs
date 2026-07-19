// 数据库迁移入口：
// - 运行时启动会执行所有 pending migration
// - 后续表结构变化只需要追加新的 mYYYYMMDD_NNNNNN_xxx 模块

use sea_orm_migration::prelude::*;

mod m20260616_000001_initial_schema;
mod m20260616_000002_add_transfer_job_success_lookup_idx;
pub(crate) mod runtime_schema;

/// SeaORM migration 注册器。
pub(crate) struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260616_000001_initial_schema::Migration),
            Box::new(m20260616_000002_add_transfer_job_success_lookup_idx::Migration),
        ]
    }
}
