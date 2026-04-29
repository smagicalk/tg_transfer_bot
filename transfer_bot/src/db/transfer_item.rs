// transfer_item 实体：
// 记录任务内每条源消息的处理状态。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_item")]
pub struct Model {
    /// 主键，自增子项 ID。
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 所属主任务 ID（`transfer_job.id`）。
    #[sea_orm(indexed)]
    pub job_id: i64,
    /// 爬虫侧：源消息 chat_id。
    pub source_chat_id: i64,
    /// 爬虫侧：源消息 message_id。
    pub source_message_id: i64,
    /// 文件去重键（优先 `remote.unique_id`，无文件时可退化为文本键）。
    pub file_key: String,
    /// 子项状态：`pending/preparing/prepared/uploading/success/failed/cancelled` 等。
    #[sea_orm(indexed)]
    pub status: String,
    /// 子项重试次数（预留字段）。
    pub retry_count: i32,
    /// 子项错误信息（失败时记录）。
    pub error_message: Option<String>,
    /// 创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
