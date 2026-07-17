// transfer_job 实体：
// 记录一次 `/transfer` 请求级别的生命周期。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_job")]
pub struct Model {
    /// 主键，自增任务 ID。
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 请求侧：发起 `/transfer` 命令所在 chat_id。
    #[sea_orm(indexed)]
    pub request_chat_id: i64,
    /// 请求侧：发起命令那条消息的 message_id。
    #[sea_orm(indexed)]
    pub request_message_id: i64,
    /// 创建任务时的所有者用户 ID，保留用于审计。
    #[sea_orm(indexed)]
    pub owner_user_id: i64,
    /// 爬虫侧：输入的源链接（抓取入口）。
    #[sea_orm(indexed)]
    pub source_link: String,
    /// 源输入类型：`link` 表示 Telegram 链接，`bot_message` 表示 bot 收到/被回复的消息。
    pub source_kind: String,
    /// 实际读取源消息的 client 角色：`bot` 或 `user`。
    pub source_client_role: String,
    /// 链接源是否允许 bot 失败后 fallback 到 user。
    pub allow_user_fallback: bool,
    /// 爬虫侧：源消息所属 chat_id。
    pub source_chat_id: i64,
    /// 爬虫侧：源入口消息的 message_id。
    pub source_message_id: i64,
    /// 爬虫侧：源相册 ID；非相册场景为 0。
    pub source_album_id: i64,
    /// 目标转存 chat_id。
    pub target_chat_id: i64,
    /// 上传结果入口消息 ID（单条消息即自身，相册则保存首条消息）。
    pub result_message_id: Option<i64>,
    /// 上传结果入口消息链接（单条消息或整组相册入口链接）。
    pub result_message_link: Option<String>,
    /// 任务状态：`pending/running/paused/cancelling/cancel_finalizing/cancelled/success/failed/partial` 等。
    #[sea_orm(indexed)]
    pub status: String,
    /// 任务总条目数（通常等于相册消息数，单条消息则为 1）。
    pub total_items: i32,
    /// 已成功完成的条目数。
    pub done_items: i32,
    /// 失败条目数。
    pub failed_items: i32,
    /// 任务级重试次数（预留字段）。
    pub retry_count: i32,
    /// 任务最后一次错误信息（可为空）。
    pub last_error: Option<String>,
    /// 创建时间（固定时区时间戳）。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    /// 完成时间（任务结束后写入）。
    pub finished_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl ActiveModelBehavior for ActiveModel {}
