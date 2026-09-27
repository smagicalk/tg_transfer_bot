//! 动态授权用户白名单数据表实体模型。
//!
//! 记录所有被所有者或管理员显式授予机器人使用权限的 Telegram 用户账号及资料快照。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// 动态授权用户的数据库持久化记录模型。
#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "authorized_user")]
pub struct Model {
    /// Telegram 用户 ID，同时作为主键避免重复授权。
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i64,
    /// Telegram 用户昵称（First Name + Last Name）；无法查询资料时允许为空。
    pub display_name: Option<String>,
    /// Telegram 用户名（不含 `@` 前缀）；用户未设置或无法查询时为空。
    pub username: Option<String>,
    /// 首次录入授权的记录时间戳。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
