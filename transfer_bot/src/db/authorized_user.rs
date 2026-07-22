// authorized_user 实体：保存由 owner 动态授权的 Telegram 用户。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "authorized_user")]
pub struct Model {
    /// Telegram user ID，同时作为主键避免重复授权。
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i64,
    /// Telegram 用户显示名称；无法查询资料时允许为空。
    pub display_name: Option<String>,
    /// Telegram 用户名（不含 `@`）；无法查询或用户没有用户名时为空。
    pub username: Option<String>,
    /// 首次授权时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
