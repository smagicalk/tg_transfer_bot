//! 交互式菜单分步输入草稿状态数据表实体模型。
//!
//! 持久化 `/menu` 导航面板中尚未完成的分步交互输入（如等待用户发送链接、选择目标频道等），
//! 即使机器人发生重启，用户发送后续文本或点击按钮仍能无缝衔接当前步骤。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// 菜单交互分步输入草稿的数据库模型。
#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "menu_input_draft")]
pub struct Model {
    /// 管理私聊会话的 Telegram Chat ID。
    #[sea_orm(primary_key, auto_increment = false)]
    pub request_chat_id: i64,
    /// 发起该输入流程的操作人 Telegram User ID。
    #[sea_orm(primary_key, auto_increment = false)]
    pub sender_user_id: i64,
    /// 当前所处的交互步骤（例如 `source_link`、`target_choice`、`confirm` 等）。
    pub step: String,
    /// 转存或查询的输入大类（例如 `transfer`、`transfer_default`、`lookup` 等）。
    pub input_kind: Option<String>,
    /// 任务控制动作（例如 `status`、`pause`、`resume`、`stop`）。
    pub job_action: Option<String>,
    /// 用户此前已输入的原始源链接；在源链接录入阶段为空。
    pub source_link: Option<String>,
    /// 用户此前已选定的目标频道 Chat ID；在最终确认阶段使用。
    pub target_chat_id: Option<i64>,
    /// 输入草稿首次创建的时间戳。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 草稿最近一次发生步骤推进或修改的时间戳。
    #[sea_orm(indexed)]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    /// 草稿的绝对过期时间戳；过期未完成时将被自动清理。
    #[sea_orm(indexed)]
    pub expires_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
