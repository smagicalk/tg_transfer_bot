//! 转存主任务执行状态与元数据实体模型。
//!
//! 记录一次完整转存请求（如单条消息或相册消息组）从调度创建、
//! 爬取准备、下载、上传至最终完成的端到端生命周期与统计状态。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// 转存主任务数据库持久化模型。
#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_job")]
pub struct Model {
    /// 主键，自增主任务 ID。
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 请求侧：发起 `/transfer` 命令所在的 Telegram 私聊 Chat ID。
    #[sea_orm(indexed)]
    pub request_chat_id: i64,
    /// 请求侧：用户触发命令的那条原消息 Message ID。
    #[sea_orm(indexed)]
    pub request_message_id: i64,
    /// 创建任务的操作人 Telegram User ID，保留用于审计与权限校验。
    #[sea_orm(indexed)]
    pub owner_user_id: i64,
    /// 爬虫侧：输入的源链接（抓取入口）。
    #[sea_orm(indexed)]
    pub source_link: String,
    /// 源输入类型：`link` 表示 Telegram 公开/私有链接，`bot_message` 表示向 Bot 发送/转发的消息。
    pub source_kind: String,
    /// 实际执行源消息读取与下载的客户端角色：`bot` 或 `user`。
    pub source_client_role: String,
    /// 当 Bot 客户端权限不足时是否允许自动回退到用户执行器重试。
    pub allow_user_fallback: bool,
    /// 爬虫侧：源消息所在的 Telegram Chat ID。
    pub source_chat_id: i64,
    /// 爬虫侧：源入口消息的 Message ID。
    pub source_message_id: i64,
    /// 爬虫侧：源媒体相册组 ID；单条消息非相册时为 0。
    pub source_album_id: i64,
    /// 目标转存频道或群组的 Telegram Chat ID。
    pub target_chat_id: i64,
    /// 上传结果入口消息 ID（单条消息即自身，媒体相册组则保存首条消息 ID）。
    pub result_message_id: Option<i64>,
    /// 上传成功后的目标消息链接（支持公有频道链接与私有 `c/` 链接）。
    pub result_message_link: Option<String>,
    /// 任务状态：如 `pending`、`running`、`paused`、`cancelling`、`cancel_finalizing`、`cancelled`、`success`、`failed`、`partial`。
    #[sea_orm(indexed)]
    pub status: String,
    /// 任务包含的子项条目总数（单消息为 1，相册组为消息数）。
    pub total_items: i32,
    /// 已成功处理完成的子项总数。
    pub done_items: i32,
    /// 处理失败的子项总数。
    pub failed_items: i32,
    /// 任务级重试次数。
    pub retry_count: i32,
    /// 任务最近一次发生错误时的详细信息。
    pub last_error: Option<String>,
    /// 任务创建时间戳（东八区固定时区）。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 任务状态最后一次变动的时间戳。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    /// 任务结束执行（成功、失败或取消）的终结时间戳。
    pub finished_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl ActiveModelBehavior for ActiveModel {}
