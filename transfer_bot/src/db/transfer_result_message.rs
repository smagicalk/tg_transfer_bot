// transfer_result_message 实体：
// 记录一次转存任务产生的所有结果入口。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_result_message")]
pub struct Model {
    /// 主键，自增结果记录 ID。
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 所属主任务 ID（`transfer_job.id`）。
    #[sea_orm(indexed)]
    pub job_id: i64,
    /// 结果分组序号，从 0 开始；超过 10 条媒体会产生多个 album 分组。
    #[sea_orm(indexed)]
    pub result_index: i32,
    /// 目标转存 chat_id。
    pub target_chat_id: i64,
    /// 结果入口消息 ID；album 保存该分组首条消息 ID。
    pub message_id: i64,
    /// 结果入口链接；无法生成可点击链接时保存可复制定位信息。
    pub message_link: String,
    /// 该结果入口是否来自 album。
    pub is_album: bool,
    /// 该结果分组包含的源条目数量。
    pub item_count: i32,
    /// 创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
