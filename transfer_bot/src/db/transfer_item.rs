//! 转存任务关联的子文件或子消息条目数据表实体模型。
//!
//! 记录主转存任务内包含的每一条源消息的细粒度处理进度与生命周期状态。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// 单条源消息/文件的转存子条目数据库模型。
#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_item")]
pub struct Model {
    /// 自增主键，单条子项记录唯一 ID。
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 所属主转存任务 ID（对应 `transfer_job.id`）。
    #[sea_orm(indexed)]
    pub job_id: i64,
    /// 爬虫侧：源消息所在的 Telegram Chat ID。
    pub source_chat_id: i64,
    /// 爬虫侧：源消息在源频道内的 Message ID。
    pub source_message_id: i64,
    /// 文件全局唯一识别键（优先采用 `remote.unique_id`，纯文本消息使用生成的文本摘要键）。
    pub file_key: String,
    /// 该文件归属的 TDLib 客户端角色（`bot` 或 `user`）。
    ///
    /// 相同文件在不同的 TDLib Client 实例下其下载 ID 与本地目录各异，需按角色隔离。
    pub file_owner_client_role: String,
    /// 子项当前流转状态（例如 `pending`、`preparing`、`prepared`、`uploading`、`success`、`failed`、`cancelled`）。
    #[sea_orm(indexed)]
    pub status: String,
    /// 该子项在错误发生后的重试执行计数。
    pub retry_count: i32,
    /// 子项执行失败时的具体错误日志描述。
    pub error_message: Option<String>,
    /// 该子项对应的文件缓存引用是否已经释放。
    ///
    /// 恢复对齐阶段可能提前释放已无效条目的引用计数，最终结算时用该标志防止重复扣减。
    pub file_ref_released: bool,
    /// 子项创建入库时间戳。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 子项状态最后更新时间戳。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
