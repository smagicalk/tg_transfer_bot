// 运行时目标配置持久化：
// - 启动时从数据库加载；不存在则用 config.json 默认值初始化
// - 当前第一版只负责统一读取来源，不开放单独命令修改
// - 提供显式连接版本，便于 PostgreSQL 启动链路测试复用真实 seed 逻辑

use sea_orm::sea_query::OnConflict;
use sea_orm::{EntityTrait, QueryOrder};

use crate::config::TargetsConfig;
use crate::db;

/// 目标配置单行主键 ID。
const TARGET_CONFIG_ROW_ID: i32 = 1;

/// 统一生成 UTC+8 时间戳。
fn now_utc8() -> chrono::DateTime<chrono::FixedOffset> {
    let Some(offset) = chrono::FixedOffset::east_opt(8 * 3600) else {
        tracing::error!("failed to build targets config UTC+8 fixed offset, fallback to UTC");
        return chrono::Utc::now().fixed_offset();
    };
    chrono::Utc::now().with_timezone(&offset)
}

/// 确保数据库中存在目标配置；若不存在则使用默认配置初始化。
///
/// # 参数
/// - `default_config`: 默认目标配置引用。
///
/// # 返回值
/// - 最终生效的目标配置。
#[cfg(test)]
pub(crate) async fn ensure_targets_runtime_config(
    default_config: &TargetsConfig,
) -> anyhow::Result<TargetsConfig> {
    ensure_targets_runtime_config_on(db::get_db().await?, default_config).await
}

/// 在显式数据库连接上确保 targets 运行态存在。
///
/// # 参数
/// - `db_conn`: 目标数据库连接。
/// - `default_config`: 默认目标配置。
///
/// # 返回值
/// - 最终生效的目标配置。
pub(crate) async fn ensure_targets_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
    default_config: &TargetsConfig,
) -> anyhow::Result<TargetsConfig> {
    if let Some(config) = load_targets_runtime_config_on(db_conn).await? {
        return Ok(config);
    }

    save_targets_runtime_config_on(db_conn, default_config).await?;
    Ok(default_config.clone())
}

/// 从数据库中读取 targets 运行时配置。
///
/// # 返回值
/// - 读出的 `TargetsConfig` 或 `None`。
#[cfg(test)]
pub(crate) async fn load_targets_runtime_config() -> anyhow::Result<Option<TargetsConfig>> {
    load_targets_runtime_config_on(db::get_db().await?).await
}

/// 在显式数据库连接上读取 targets 运行态（包含默认目标与别名映射表）。
///
/// # 参数
/// - `db_conn`: 数据库连接。
///
/// # 返回值
/// - 查出的 `TargetsConfig` 或 `None`。
pub(crate) async fn load_targets_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
) -> anyhow::Result<Option<TargetsConfig>> {
    let default_chat_id = db::transfer_target_config::Entity::find_by_id(TARGET_CONFIG_ROW_ID)
        .one(db_conn)
        .await?
        .map(|row| row.default_chat_id)
        .unwrap_or(0);

    let aliases = db::transfer_target_alias::Entity::find()
        .order_by_asc(db::transfer_target_alias::Column::Alias)
        .all(db_conn)
        .await?
        .into_iter()
        .map(|row| (row.alias, row.target_chat_id))
        .collect::<std::collections::HashMap<_, _>>();

    let config = TargetsConfig {
        default_chat_id,
        aliases,
    };

    if config.is_empty() {
        Ok(None)
    } else {
        Ok(Some(config))
    }
}

/// 保存 targets 运行时配置到数据库。
///
/// # 参数
/// - `config`: 新目标配置。
pub(crate) async fn save_targets_runtime_config(config: &TargetsConfig) -> anyhow::Result<()> {
    save_targets_runtime_config_on(db::get_db().await?, config).await
}

/// 在显式数据库连接上写回 targets 运行态（写入默认目标并全量刷新别名表）。
///
/// # 参数
/// - `db_conn`: 数据库连接。
/// - `config`: 新目标配置。
pub(crate) async fn save_targets_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
    config: &TargetsConfig,
) -> anyhow::Result<()> {
    let now = now_utc8();

    db::transfer_target_config::Entity::insert(db::transfer_target_config::ActiveModel {
        id: sea_orm::ActiveValue::Set(TARGET_CONFIG_ROW_ID),
        default_chat_id: sea_orm::ActiveValue::Set(config.default_chat_id),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
    })
    .on_conflict(
        OnConflict::column(db::transfer_target_config::Column::Id)
            .update_columns([
                db::transfer_target_config::Column::DefaultChatId,
                db::transfer_target_config::Column::UpdatedAt,
            ])
            .to_owned(),
    )
    .exec_without_returning(db_conn)
    .await?;

    db::transfer_target_alias::Entity::delete_many()
        .exec(db_conn)
        .await?;
    for (alias, target_chat_id) in &config.aliases {
        db::transfer_target_alias::Entity::insert(db::transfer_target_alias::ActiveModel {
            alias: sea_orm::ActiveValue::Set(alias.clone()),
            target_chat_id: sea_orm::ActiveValue::Set(*target_chat_id),
            created_at: sea_orm::ActiveValue::Set(now),
            updated_at: sea_orm::ActiveValue::Set(now),
        })
        .exec_without_returning(db_conn)
        .await?;
    }

    Ok(())
}
