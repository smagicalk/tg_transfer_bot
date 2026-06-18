// transfer_target_route 实体：
// 保存 request_chat_id -> target_chat_id 的默认映射。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_target_route")]
pub struct Model {
    /// 请求 chat 主键。
    #[sea_orm(primary_key, auto_increment = false)]
    pub request_chat_id: i64,
    /// 对应默认目标 chat。
    pub target_chat_id: i64,
    /// 配置记录创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 配置记录最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
