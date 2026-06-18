// billing_runtime_config 实体：
// 持久化计费规则与首页公告等可运行时读取的业务配置。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "billing_runtime_config")]
pub struct Model {
    /// 单行配置主键，固定为 1。
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    /// 是否启用普通用户积分计费。
    pub enabled: bool,
    /// 每次转存基础积分成本。
    pub base_cost_points: i64,
    /// 每个条目的附加积分成本。
    pub item_cost_points: i64,
    /// 用户首次接触机器人时发放的初始积分。
    pub initial_user_points: i64,
    /// 首页公告文本。
    pub announcement_text: Option<String>,
    /// 配置记录创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 配置记录最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
