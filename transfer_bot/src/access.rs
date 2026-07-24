// 动态授权名单持久化：命令写数据库后再同步 AppContext 运行态。

use std::collections::BTreeSet;

use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, QueryOrder};

use crate::db;

/// 读取全部动态授权用户，按 user ID 排序返回。
pub(crate) async fn list_authorized_user_ids_on(
    db_conn: &sea_orm::DatabaseConnection,
) -> anyhow::Result<BTreeSet<i64>> {
    Ok(list_authorized_users_on(db_conn)
        .await?
        .into_iter()
        .map(|row| row.user_id)
        .filter(|user_id| *user_id > 0)
        .collect())
}

/// 读取全部动态授权用户及其名称快照，按 user ID 排序返回。
///
/// 返回数据库实体而不是只返回 ID，便于授权管理界面同时展示名称、用户名和 ID。
pub(crate) async fn list_authorized_users_on(
    db_conn: &sea_orm::DatabaseConnection,
) -> anyhow::Result<Vec<db::authorized_user::Model>> {
    Ok(db::authorized_user::Entity::find()
        .order_by_asc(db::authorized_user::Column::UserId)
        .all(db_conn)
        .await?
        .into_iter()
        .filter(|row| row.user_id > 0)
        .collect())
}

/// 持久化单个授权；已存在时保持幂等并返回 false。
pub(crate) async fn grant_authorized_user_on(
    db_conn: &sea_orm::DatabaseConnection,
    user_id: i64,
) -> anyhow::Result<bool> {
    grant_authorized_user_with_profile_on(db_conn, user_id, None, None).await
}

/// 持久化单个授权并保存可选的 Telegram 名称资料。
///
/// 已存在的授权仍返回 `false`；当调用方提供了非空资料时，会顺便刷新已有记录中的
/// 对应字段。这样原有 ID-only 命令保持幂等，新用户选择器则可补齐名称快照。
pub(crate) async fn grant_authorized_user_with_profile_on(
    db_conn: &sea_orm::DatabaseConnection,
    user_id: i64,
    display_name: Option<&str>,
    username: Option<&str>,
) -> anyhow::Result<bool> {
    validate_user_id(user_id)?;
    let normalized_display_name = normalize_display_name(display_name);
    let normalized_username = normalize_username(username);

    if let Some(existing) = db::authorized_user::Entity::find_by_id(user_id)
        .one(db_conn)
        .await?
    {
        // Legacy callers pass None/None; do not erase a profile already saved by the picker.
        let should_update_display_name =
            display_name.is_some() && existing.display_name != normalized_display_name;
        let should_update_username = username.is_some() && existing.username != normalized_username;
        if should_update_display_name || should_update_username {
            let mut active = existing.into_active_model();
            if should_update_display_name {
                active.display_name = sea_orm::ActiveValue::Set(normalized_display_name);
            }
            if should_update_username {
                active.username = sea_orm::ActiveValue::Set(normalized_username);
            }
            active.update(db_conn).await?;
        }
        return Ok(false);
    }

    db::authorized_user::Entity::insert(db::authorized_user::ActiveModel {
        user_id: sea_orm::ActiveValue::Set(user_id),
        display_name: sea_orm::ActiveValue::Set(normalized_display_name),
        username: sea_orm::ActiveValue::Set(normalized_username),
        created_at: sea_orm::ActiveValue::Set(now_utc8()),
    })
    .on_conflict(
        OnConflict::column(db::authorized_user::Column::UserId)
            .do_nothing()
            .to_owned(),
    )
    .exec_without_returning(db_conn)
    .await?;
    Ok(true)
}

/// 更新已有动态授权用户的名称资料。
///
/// `None` 表示清除对应字段；若用户尚未授权则返回 `false`，不隐式创建授权记录。
pub(crate) async fn update_authorized_user_profile_on(
    db_conn: &sea_orm::DatabaseConnection,
    user_id: i64,
    display_name: Option<&str>,
    username: Option<&str>,
) -> anyhow::Result<bool> {
    validate_user_id(user_id)?;
    let Some(existing) = db::authorized_user::Entity::find_by_id(user_id)
        .one(db_conn)
        .await?
    else {
        return Ok(false);
    };

    let normalized_display_name = normalize_display_name(display_name);
    let normalized_username = normalize_username(username);
    if existing.display_name == normalized_display_name && existing.username == normalized_username
    {
        return Ok(true);
    }

    let mut active = existing.into_active_model();
    active.display_name = sea_orm::ActiveValue::Set(normalized_display_name);
    active.username = sea_orm::ActiveValue::Set(normalized_username);
    active.update(db_conn).await?;
    Ok(true)
}

/// 删除单个动态授权；不存在时保持幂等并返回 false。
pub(crate) async fn revoke_authorized_user_on(
    db_conn: &sea_orm::DatabaseConnection,
    user_id: i64,
) -> anyhow::Result<bool> {
    validate_user_id(user_id)?;
    let result = db::authorized_user::Entity::delete_by_id(user_id)
        .exec(db_conn)
        .await?;
    Ok(result.rows_affected > 0)
}

fn validate_user_id(user_id: i64) -> anyhow::Result<()> {
    if user_id <= 0 {
        anyhow::bail!("user_id must be positive");
    }
    Ok(())
}

fn normalize_display_name(value: Option<&str>) -> Option<String> {
    normalize_optional_text(value)
}

fn normalize_username(value: Option<&str>) -> Option<String> {
    normalize_optional_text(value)
        .map(|username| username.trim_start_matches('@').trim().to_owned())
        .filter(|username| !username.is_empty())
}

fn normalize_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn now_utc8() -> chrono::DateTime<chrono::FixedOffset> {
    let Some(offset) = chrono::FixedOffset::east_opt(8 * 3600) else {
        tracing::error!("failed to build access UTC+8 fixed offset, fallback to UTC");
        return chrono::Utc::now().fixed_offset();
    };
    chrono::Utc::now().with_timezone(&offset)
}
