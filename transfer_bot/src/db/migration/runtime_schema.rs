// 运行时业务 schema 定义：
// - 按业务域拆成多个子模块，便于后续 migration 继续增量扩展
// - migration 文件只负责声明版本和调用这些 helper，避免把大量 DDL 塞进单个 migration 文件

mod access;
mod cache;
mod menu;
mod runtime_config;
mod transfer;

use sea_orm::ConnectionTrait;
use sea_orm::StatementBuilder;

/// 创建当前版本所需的全部业务表与索引。
pub(crate) async fn create_runtime_schema<C>(db: &C) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    access::create(db).await?;
    transfer::create(db).await?;
    cache::create(db).await?;
    menu::create(db).await?;
    runtime_config::create(db).await?;
    Ok(())
}

/// 按依赖反序删表；测试重建和 migration down 共用。
pub(crate) async fn drop_runtime_schema<C>(db: &C) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    runtime_config::drop(db).await?;
    menu::drop(db).await?;
    cache::drop(db).await?;
    transfer::drop(db).await?;
    access::drop(db).await?;
    Ok(())
}

pub(crate) async fn drop_access_schema<C>(db: &C) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    access::drop(db).await
}

/// 执行单条 schema builder。
async fn exec_schema_statement<S>(db: &impl ConnectionTrait, statement: S) -> anyhow::Result<()>
where
    S: StatementBuilder,
{
    db.execute(&statement).await?;
    Ok(())
}
