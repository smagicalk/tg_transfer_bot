// 运行时访问控制配置持久化：
// - 启动时从数据库加载；不存在则用 config.json 默认值初始化
// - 当前第一版只负责统一读取来源，不开放单独命令修改
// - 提供显式连接版本，便于 PostgreSQL 启动链路测试复用真实 seed 逻辑

use sea_orm::sea_query::OnConflict;
use sea_orm::{EntityTrait, QueryOrder};

use crate::config::AccessControlConfig;
use crate::db;

const ACCESS_CONTROL_CONFIG_ROW_ID: i32 = 1;

fn now_utc8() -> chrono::DateTime<chrono::FixedOffset> {
    let Some(offset) = chrono::FixedOffset::east_opt(8 * 3600) else {
        tracing::error!("failed to build access control UTC+8 fixed offset, fallback to UTC");
        return chrono::Utc::now().fixed_offset();
    };
    chrono::Utc::now().with_timezone(&offset)
}

#[cfg(test)]
pub(crate) async fn ensure_access_control_runtime_config(
    default_config: &AccessControlConfig,
) -> anyhow::Result<AccessControlConfig> {
    ensure_access_control_runtime_config_on(db::get_db().await?, default_config).await
}

/// 在显式数据库连接上确保访问控制运行态存在。
pub(crate) async fn ensure_access_control_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
    default_config: &AccessControlConfig,
) -> anyhow::Result<AccessControlConfig> {
    if let Some(config) = load_access_control_runtime_config_on(db_conn).await? {
        return Ok(config);
    }

    save_access_control_runtime_config_on(db_conn, default_config).await?;
    Ok(default_config.clone())
}

#[cfg(test)]
pub(crate) async fn load_access_control_runtime_config()
-> anyhow::Result<Option<AccessControlConfig>> {
    load_access_control_runtime_config_on(db::get_db().await?).await
}

/// 在显式数据库连接上读取访问控制运行态。
pub(crate) async fn load_access_control_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
) -> anyhow::Result<Option<AccessControlConfig>> {
    let allow_all_private_users =
        db::access_control_runtime_config::Entity::find_by_id(ACCESS_CONTROL_CONFIG_ROW_ID)
            .one(db_conn)
            .await?
            .map(|row| row.allow_all_private_users)
            .unwrap_or(false);

    let admin_user_ids = db::access_control_admin_user::Entity::find()
        .order_by_asc(db::access_control_admin_user::Column::TelegramUserId)
        .all(db_conn)
        .await?
        .into_iter()
        .map(|row| row.telegram_user_id)
        .collect::<Vec<_>>();

    let allowed_user_ids = db::access_control_allowed_user::Entity::find()
        .order_by_asc(db::access_control_allowed_user::Column::TelegramUserId)
        .all(db_conn)
        .await?
        .into_iter()
        .map(|row| row.telegram_user_id)
        .collect::<Vec<_>>();

    let banned_user_ids = db::access_control_banned_user::Entity::find()
        .order_by_asc(db::access_control_banned_user::Column::TelegramUserId)
        .all(db_conn)
        .await?
        .into_iter()
        .map(|row| row.telegram_user_id)
        .collect::<Vec<_>>();

    let allowed_request_chat_ids = db::access_control_allowed_request_chat::Entity::find()
        .order_by_asc(db::access_control_allowed_request_chat::Column::ChatId)
        .all(db_conn)
        .await?
        .into_iter()
        .map(|row| row.chat_id)
        .collect::<Vec<_>>();

    let allowed_target_chat_ids = db::access_control_allowed_target_chat::Entity::find()
        .order_by_asc(db::access_control_allowed_target_chat::Column::ChatId)
        .all(db_conn)
        .await?
        .into_iter()
        .map(|row| row.chat_id)
        .collect::<Vec<_>>();

    let config = AccessControlConfig {
        bootstrap_admin_user_ids: Vec::new(),
        admin_user_ids,
        allowed_user_ids,
        allow_all_private_users,
        banned_user_ids,
        allowed_request_chat_ids,
        allowed_target_chat_ids,
    };

    if config.admin_user_ids.is_empty()
        && config.allowed_user_ids.is_empty()
        && !config.allow_all_private_users
        && config.banned_user_ids.is_empty()
        && config.allowed_request_chat_ids.is_empty()
        && config.allowed_target_chat_ids.is_empty()
    {
        Ok(None)
    } else {
        Ok(Some(config))
    }
}

pub(crate) async fn save_access_control_runtime_config(
    config: &AccessControlConfig,
) -> anyhow::Result<()> {
    save_access_control_runtime_config_on(db::get_db().await?, config).await
}

/// 在显式数据库连接上写回访问控制运行态。
pub(crate) async fn save_access_control_runtime_config_on(
    db_conn: &sea_orm::DatabaseConnection,
    config: &AccessControlConfig,
) -> anyhow::Result<()> {
    let now = now_utc8();

    db::access_control_runtime_config::Entity::insert(
        db::access_control_runtime_config::ActiveModel {
            id: sea_orm::ActiveValue::Set(ACCESS_CONTROL_CONFIG_ROW_ID),
            allow_all_private_users: sea_orm::ActiveValue::Set(config.allow_all_private_users),
            created_at: sea_orm::ActiveValue::Set(now),
            updated_at: sea_orm::ActiveValue::Set(now),
        },
    )
    .on_conflict(
        OnConflict::column(db::access_control_runtime_config::Column::Id)
            .update_columns([
                db::access_control_runtime_config::Column::AllowAllPrivateUsers,
                db::access_control_runtime_config::Column::UpdatedAt,
            ])
            .to_owned(),
    )
    .exec_without_returning(db_conn)
    .await?;

    db::access_control_admin_user::Entity::delete_many()
        .exec(db_conn)
        .await?;
    for telegram_user_id in &config.admin_user_ids {
        db::access_control_admin_user::Entity::insert(db::access_control_admin_user::ActiveModel {
            telegram_user_id: sea_orm::ActiveValue::Set(*telegram_user_id),
            created_at: sea_orm::ActiveValue::Set(now),
            updated_at: sea_orm::ActiveValue::Set(now),
        })
        .exec_without_returning(db_conn)
        .await?;
    }

    db::access_control_allowed_user::Entity::delete_many()
        .exec(db_conn)
        .await?;
    for telegram_user_id in &config.allowed_user_ids {
        db::access_control_allowed_user::Entity::insert(
            db::access_control_allowed_user::ActiveModel {
                telegram_user_id: sea_orm::ActiveValue::Set(*telegram_user_id),
                created_at: sea_orm::ActiveValue::Set(now),
                updated_at: sea_orm::ActiveValue::Set(now),
            },
        )
        .exec_without_returning(db_conn)
        .await?;
    }

    db::access_control_banned_user::Entity::delete_many()
        .exec(db_conn)
        .await?;
    for telegram_user_id in &config.banned_user_ids {
        db::access_control_banned_user::Entity::insert(
            db::access_control_banned_user::ActiveModel {
                telegram_user_id: sea_orm::ActiveValue::Set(*telegram_user_id),
                created_at: sea_orm::ActiveValue::Set(now),
                updated_at: sea_orm::ActiveValue::Set(now),
            },
        )
        .exec_without_returning(db_conn)
        .await?;
    }

    db::access_control_allowed_request_chat::Entity::delete_many()
        .exec(db_conn)
        .await?;
    for chat_id in &config.allowed_request_chat_ids {
        db::access_control_allowed_request_chat::Entity::insert(
            db::access_control_allowed_request_chat::ActiveModel {
                chat_id: sea_orm::ActiveValue::Set(*chat_id),
                created_at: sea_orm::ActiveValue::Set(now),
                updated_at: sea_orm::ActiveValue::Set(now),
            },
        )
        .exec_without_returning(db_conn)
        .await?;
    }

    db::access_control_allowed_target_chat::Entity::delete_many()
        .exec(db_conn)
        .await?;
    for chat_id in &config.allowed_target_chat_ids {
        db::access_control_allowed_target_chat::Entity::insert(
            db::access_control_allowed_target_chat::ActiveModel {
                chat_id: sea_orm::ActiveValue::Set(*chat_id),
                created_at: sea_orm::ActiveValue::Set(now),
                updated_at: sea_orm::ActiveValue::Set(now),
            },
        )
        .exec_without_returning(db_conn)
        .await?;
    }

    Ok(())
}
