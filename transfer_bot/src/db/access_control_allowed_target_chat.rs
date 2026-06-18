// access_control_allowed_target_chat 实体：
// 保存允许作为转存目标的 chat 列表。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "access_control_allowed_target_chat")]
pub struct Model {
    /// 允许的目标 chat_id。
    #[sea_orm(primary_key, auto_increment = false)]
    pub chat_id: i64,
    /// 记录创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 记录最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
