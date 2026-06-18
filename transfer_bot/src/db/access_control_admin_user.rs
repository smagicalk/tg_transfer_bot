// access_control_admin_user 实体：
// 保存数据库层扩展管理员用户列表。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "access_control_admin_user")]
pub struct Model {
    /// 管理员 Telegram user_id。
    #[sea_orm(primary_key, auto_increment = false)]
    pub telegram_user_id: i64,
    /// 记录创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 记录最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
