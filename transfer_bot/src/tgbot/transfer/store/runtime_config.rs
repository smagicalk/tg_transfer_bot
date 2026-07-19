// 运行时转存配置持久化：
// - 启动时从数据库加载；不存在则用 config.json 默认值初始化
// - `/config` 修改时写回数据库并立即刷新内存运行态
// - 提供显式连接版本，便于 PostgreSQL 启动链路测试直接复用真实 seed 逻辑

use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::config::TransferConfig;
use crate::db;

const RUNTIME_CONFIG_ROW_ID: i32 = 1;

/// 统一生成 UTC+8 时间戳。
fn now_utc8() -> chrono::DateTime<chrono::FixedOffset> {
    let Some(offset) = chrono::FixedOffset::east_opt(8 * 3600) else {
        tracing::error!("failed to build runtime config UTC+8 fixed offset, fallback to UTC");
        return chrono::Utc::now().fixed_offset();
    };
    chrono::Utc::now().with_timezone(&offset)
}

/// 启动时确保数据库里存在一份运行参数。
/// 如果表里没有记录，则把 config.json 里的默认值写进去。
#[cfg(test)]
pub(crate) async fn ensure_transfer_runtime_config(
    default_config: &TransferConfig,
) -> anyhow::Result<TransferConfig> {
    ensure_transfer_runtime_config_on(db::get_db().await?, default_config).await
}

/// 在显式数据库连接上确保运行参数单行存在。
///
/// 正常运行走全局连接池；测试 PostgreSQL 启动链路时会直接传入独立连接，
/// 这样验证的就是和启动同一套 seed 逻辑，而不是另一份测试专用分支。
pub(crate) async fn ensure_transfer_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
    default_config: &TransferConfig,
) -> anyhow::Result<TransferConfig> {
    if let Some(model) = load_transfer_runtime_config_on(db_conn).await? {
        return TransferConfig::from_db_model(&model);
    }

    save_transfer_runtime_config_on(db_conn, default_config).await?;
    Ok(default_config.clone())
}

/// 读取数据库中的单行运行参数。
#[cfg(test)]
pub(crate) async fn load_transfer_runtime_config()
-> anyhow::Result<Option<db::transfer_runtime_config::Model>> {
    load_transfer_runtime_config_on(db::get_db().await?).await
}

/// 在显式数据库连接上读取单行运行参数。
pub(crate) async fn load_transfer_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
) -> anyhow::Result<Option<db::transfer_runtime_config::Model>> {
    Ok(db::transfer_runtime_config::Entity::find()
        .filter(db::transfer_runtime_config::Column::Id.eq(RUNTIME_CONFIG_ROW_ID))
        .one(db_conn)
        .await?)
}

/// 写回数据库中的运行参数。
pub(crate) async fn save_transfer_runtime_config(config: &TransferConfig) -> anyhow::Result<()> {
    save_transfer_runtime_config_on(db::get_db().await?, config).await
}

/// 在显式数据库连接上写回运行参数。
pub(crate) async fn save_transfer_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
    config: &TransferConfig,
) -> anyhow::Result<()> {
    let now = now_utc8();
    db::transfer_runtime_config::Entity::insert(config.to_db_row(now))
        .on_conflict(
            OnConflict::column(db::transfer_runtime_config::Column::Id)
                .update_columns([
                    db::transfer_runtime_config::Column::JobConcurrency,
                    db::transfer_runtime_config::Column::FileDeleteDelayMinutes,
                    db::transfer_runtime_config::Column::FileGcIntervalSeconds,
                    db::transfer_runtime_config::Column::ProgressEditIntervalSeconds,
                    db::transfer_runtime_config::Column::DownloadsDefaultPageSize,
                    db::transfer_runtime_config::Column::MenuInputTimeoutSeconds,
                    db::transfer_runtime_config::Column::UpdatedAt,
                ])
                .to_owned(),
        )
        .exec_without_returning(db_conn)
        .await?;
    Ok(())
}
