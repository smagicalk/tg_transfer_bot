//! 转存完成发送的目标卡片消息定位表实体模型。
//!
//! 记录一次转存任务成功上传后在目标频道中生成的所有结果入口消息。
//! Telegram 媒体相册存在上限 10 张的限制，超过时会产生多个分段，每个分段均在此记录其入口与链接。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// 转存目标频道结果消息定位数据库模型。
#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_result_message")]
pub struct Model {
    /// 主键，自增结果记录 ID。
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 所属主转存任务 ID（对应 `transfer_job.id`）。
    #[sea_orm(indexed)]
    pub job_id: i64,
    /// 结果分组序号，从 0 开始自增；超过 10 条媒体会拆分为多个 album 分组。
    #[sea_orm(indexed)]
    pub result_index: i32,
    /// 目标转存频道或群组的 Telegram Chat ID。
    pub target_chat_id: i64,
    /// 结果入口消息 ID；媒体相册组保存该分组的首条消息 ID。
    pub message_id: i64,
    /// 结果入口可跳转链接；若为无公网名的私有群组则保存可定位的 `c/` 链接。
    pub message_link: String,
    /// 该结果入口是否为媒体相册（Album）。
    pub is_album: bool,
    /// 该分组内聚合包含的子条目总数。
    pub item_count: i32,
    /// 记录生成时间戳。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 记录最近更新时间戳。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
