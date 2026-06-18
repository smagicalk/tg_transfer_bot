// 运行时计费配置持久化：
// - 启动时从数据库加载；不存在则用 config.json 默认值初始化
// - 当前第一版只负责统一读取来源，不开放单独命令修改
// - 提供显式连接版本，便于 PostgreSQL 启动链路测试复用真实 seed 逻辑

use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::config::BillingConfig;
use crate::db;

const BILLING_CONFIG_ROW_ID: i32 = 1;

fn now_utc8() -> chrono::DateTime<chrono::FixedOffset> {
    let Some(offset) = chrono::FixedOffset::east_opt(8 * 3600) else {
        tracing::error!("failed to build billing config UTC+8 fixed offset, fallback to UTC");
        return chrono::Utc::now().fixed_offset();
    };
    chrono::Utc::now().with_timezone(&offset)
}

#[cfg(test)]
pub(crate) async fn ensure_billing_runtime_config(
    default_config: &BillingConfig,
) -> anyhow::Result<BillingConfig> {
    ensure_billing_runtime_config_on(db::get_db().await?, default_config).await
}

/// 在显式数据库连接上确保计费运行态存在。
pub(crate) async fn ensure_billing_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
    default_config: &BillingConfig,
) -> anyhow::Result<BillingConfig> {
    if let Some(model) = load_billing_runtime_config_on(db_conn).await? {
        return Ok(BillingConfig::from_db_model(&model));
    }

    save_billing_runtime_config_on(db_conn, default_config).await?;
    Ok(default_config.clone())
}

#[cfg(test)]
pub(crate) async fn load_billing_runtime_config()
-> anyhow::Result<Option<db::billing_runtime_config::Model>> {
    load_billing_runtime_config_on(db::get_db().await?).await
}

/// 在显式数据库连接上读取计费运行态。
pub(crate) async fn load_billing_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
) -> anyhow::Result<Option<db::billing_runtime_config::Model>> {
    Ok(db::billing_runtime_config::Entity::find()
        .filter(db::billing_runtime_config::Column::Id.eq(BILLING_CONFIG_ROW_ID))
        .one(db_conn)
        .await?)
}

pub(crate) async fn save_billing_runtime_config(config: &BillingConfig) -> anyhow::Result<()> {
    save_billing_runtime_config_on(db::get_db().await?, config).await
}

/// 在显式数据库连接上写回计费运行态。
pub(crate) async fn save_billing_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
    config: &BillingConfig,
) -> anyhow::Result<()> {
    let now = now_utc8();
    db::billing_runtime_config::Entity::insert(config.to_db_row(now))
        .on_conflict(
            OnConflict::column(db::billing_runtime_config::Column::Id)
                .update_columns([
                    db::billing_runtime_config::Column::Enabled,
                    db::billing_runtime_config::Column::BaseCostPoints,
                    db::billing_runtime_config::Column::ItemCostPoints,
                    db::billing_runtime_config::Column::InitialUserPoints,
                    db::billing_runtime_config::Column::AnnouncementText,
                    db::billing_runtime_config::Column::UpdatedAt,
                ])
                .to_owned(),
        )
        .exec_without_returning(db_conn)
        .await?;
    Ok(())
}
