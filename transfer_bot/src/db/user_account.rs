// user_account 实体：
// 记录 Telegram 用户在本机器人里的角色和积分余额。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_account")]
pub struct Model {
    /// Telegram 用户 ID，作为账号主键。
    #[sea_orm(primary_key, auto_increment = false)]
    pub telegram_user_id: i64,
    /// 业务角色：`admin/user/banned`。
    #[sea_orm(indexed)]
    pub role: String,
    /// 当前积分余额；普通用户转存前需要余额足够。
    pub points_balance: i64,
    /// 累计增加积分，用于审计 admin 充值操作。
    pub total_points_added: i64,
    /// 累计消耗积分，用于统计普通用户使用量。
    pub total_points_spent: i64,
    /// 账号首次创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 账号最近更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
