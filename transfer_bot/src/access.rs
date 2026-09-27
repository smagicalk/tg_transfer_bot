//! 动态授权白名单数据库持久化访问层。
//!
//! 提供在底层 SQLite 数据库中持久化、查询、更新及吊销用户白名单的功能。
//! 命令层（例如 `/auth`）在写入数据库成功后，会同步更新 `AppContext` 中的内存白名单。

use std::collections::BTreeSet;

use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, QueryOrder};

use crate::db;

/// 从数据库读取全部动态授权用户 ID，按升序去重排序返回集合。
///
/// 过滤掉无效的非正数 ID。
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

/// 读取全部动态授权用户的完整数据库记录实体列表，按 user_id 升序排列。
///
/// 返回数据库实体而不是单纯的 ID，便于授权管理卡片界面同时展示昵称、用户名与 ID。
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

/// 持久化单个用户授权记录；若该用户已存在授权，则保持幂等并返回 `false`。
pub(crate) async fn grant_authorized_user_on(
    db_conn: &sea_orm::DatabaseConnection,
    user_id: i64,
) -> anyhow::Result<bool> {
    grant_authorized_user_with_profile_on(db_conn, user_id, None, None).await
}

/// 持久化单个用户授权并记录其 Telegram 昵称与用户名资料快照。
///
/// - 若记录已存在：返回 `false`，但若调用方传入了新的非空昵称或用户名，会顺便更新已有记录；
/// - 若记录不存在：插入新授权记录并返回 `true`。
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
        // 旧调用方传入 None/None 时不覆盖已有资料；仅在显式提供新资料时按需刷新。
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

/// 更新已有动态授权用户的 Telegram 昵称与用户名资料快照。
///
/// 若用户尚未获得授权则返回 `false`（不隐式创建授权）；若更新成功或无变化返回 `true`。
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

/// 从数据库中删除单个用户的授权记录；若记录本就不存在则保持幂等并返回 `false`。
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

/// 校验 Telegram 用户 ID 是否为正整数。
fn validate_user_id(user_id: i64) -> anyhow::Result<()> {
    if user_id <= 0 {
        anyhow::bail!("user_id must be positive");
    }
    Ok(())
}

/// 规整可选的用户显示昵称（去除首尾空白字符，空串转为 `None`）。
fn normalize_display_name(value: Option<&str>) -> Option<String> {
    normalize_optional_text(value)
}

/// 规整可选的用户名（去除开头的 `@` 符号及首尾空白字符，空串转为 `None`）。
fn normalize_username(value: Option<&str>) -> Option<String> {
    normalize_optional_text(value)
        .map(|username| username.trim_start_matches('@').trim().to_owned())
        .filter(|username| !username.is_empty())
}

/// 辅助过滤空文本并提取为 `Option<String>`。
fn normalize_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

/// 获取当前时间的东八区（UTC+8）时间戳。
fn now_utc8() -> chrono::DateTime<chrono::FixedOffset> {
    let Some(offset) = chrono::FixedOffset::east_opt(8 * 3600) else {
        tracing::error!("failed to build access UTC+8 fixed offset, fallback to UTC");
        return chrono::Utc::now().fixed_offset();
    };
    chrono::Utc::now().with_timezone(&offset)
}
