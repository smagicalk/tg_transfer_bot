//! 运行时业务 Schema 定义与 DDL 调度模块。
//!
//! 按业务域解耦为子模块（权限、转存、缓存、菜单、配置），
//! Migration 迁移文件仅负责声明版本号并调用本模块的构建函数，避免单文件臃肿。

mod access;
mod cache;
mod menu;
mod runtime_config;
mod transfer;

use sea_orm::ConnectionTrait;
use sea_orm::StatementBuilder;

/// 批量创建当前应用程序版本所需的全部业务数据表与索引结构。
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

/// 按外键与依赖反序级联删除全部业务表；供测试重置环境与 Migration 回滚（Down）复用。
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

/// 删除授权白名单相关数据表结构。
pub(crate) async fn drop_access_schema<C>(db: &C) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    access::drop(db).await
}

/// 执行单条 DDL Schema 构建语句。
async fn exec_schema_statement<S>(db: &impl ConnectionTrait, statement: S) -> anyhow::Result<()>
where
    S: StatementBuilder,
{
    db.execute(&statement).await?;
    Ok(())
}
