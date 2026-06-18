// access_control_banned_user 实体：
// 保存禁止访问的用户列表。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "access_control_banned_user")]
pub struct Model {
    /// 被禁止的 Telegram user_id。
    #[sea_orm(primary_key, auto_increment = false)]
    pub telegram_user_id: i64,
    /// 记录创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 记录最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
