// point_ledger 实体：
// 记录每一次积分增加、扣减和未来退款，便于排查余额变化。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "point_ledger")]
pub struct Model {
    /// 自增账本 ID。
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 发生积分变更的 Telegram 用户 ID。
    #[sea_orm(indexed)]
    pub telegram_user_id: i64,
    /// 积分变化量；正数表示增加，负数表示扣减。
    pub delta: i64,
    /// 本次变更后的余额快照。
    pub balance_after: i64,
    /// 变更原因：transfer_charge/admin_adjust 等。
    pub reason: String,
    /// 关联任务 ID；admin 手动调整时可为空。
    pub job_id: Option<i64>,
    /// 关联请求 chat_id；用于定位是哪条命令触发扣费。
    pub request_chat_id: Option<i64>,
    /// 关联请求 message_id；用于同一条命令重复投递时幂等。
    pub request_message_id: Option<i64>,
    /// 幂等键；同一扣费请求重复执行时不会重复插账。
    pub idempotency_key: Option<String>,
    /// 操作者用户 ID；用户消费时是自己，admin 调整时是 admin。
    pub created_by: Option<i64>,
    /// 账本创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
