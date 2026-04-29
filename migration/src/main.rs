// migration 可执行入口。
// 用于通过 SeaORM CLI 执行迁移命令。
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
