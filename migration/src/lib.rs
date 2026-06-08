// migration crate 对外入口。
// 声明所有迁移文件，并按顺序返回给 SeaORM 执行。
pub use sea_orm_migration::prelude::*;

// 当前唯一迁移：初始化任务/子项/文件缓存表。
mod m20220101_000001_create_table;

// Migrator 是 SeaORM 约定的迁移注册器。
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    // 迁移执行顺序由此 Vec 的顺序决定。
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}
