// menu_input_draft 实体：
// 持久化 `/menu` 尚未完成的输入草稿，进程重启后仍能继续等待用户回复。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "menu_input_draft")]
pub struct Model {
    /// 管理私聊 chat_id。
    #[sea_orm(primary_key, auto_increment = false)]
    pub request_chat_id: i64,
    /// 发起输入流程的管理员 user_id。
    #[sea_orm(primary_key, auto_increment = false)]
    pub sender_user_id: i64,
    /// 当前步骤：source_link / target_choice / target_chat / chat_picker / confirm / job_id。
    pub step: String,
    /// 转存/查询输入类型：transfer / transfer_default / lookup / lookup_default。
    pub input_kind: Option<String>,
    /// 任务控制动作：status / pause / resume / stop。
    pub job_action: Option<String>,
    /// 已输入的源链接；源链接阶段为空。
    pub source_link: Option<String>,
    /// 已选择的目标 chat；确认阶段使用。
    pub target_chat_id: Option<i64>,
    /// 草稿创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 草稿最近更新时间。
    #[sea_orm(indexed)]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    /// 草稿过期时间；读取时若已过期会删除。
    #[sea_orm(indexed)]
    pub expires_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
