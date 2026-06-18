// transfer_target_alias 实体：
// 保存目标别名到 target_chat_id 的映射。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_target_alias")]
pub struct Model {
    /// 别名主键。
    #[sea_orm(primary_key, auto_increment = false)]
    pub alias: String,
    /// 别名对应的目标 chat。
    pub target_chat_id: i64,
    /// 配置记录创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 配置记录最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
