// transfer_target_config 实体：
// 保存目标配置中的默认目标 chat。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_target_config")]
pub struct Model {
    /// 单行配置主键，固定为 1。
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    /// 默认目标 chat；0 表示没有默认目标。
    pub default_chat_id: i64,
    /// 配置记录创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 配置记录最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
