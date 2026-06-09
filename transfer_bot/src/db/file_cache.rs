// file_cache 实体：
// 以 file_key 做跨任务下载去重、引用计数和延迟删除队列。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "file_cache")]
pub struct Model {
    /// 文件所属 TDLib client 角色：`bot` 或 `user`。
    #[sea_orm(primary_key, auto_increment = false)]
    pub owner_client_role: String,
    /// 文件缓存主键：全局 file_key（跨任务去重）。
    #[sea_orm(primary_key, auto_increment = false)]
    pub file_key: String,
    /// 缓存状态：`downloading/ready/failed` 等。
    #[sea_orm(indexed)]
    pub status: String,
    /// 文件大小（字节，已知时写入）。
    pub size_bytes: Option<i64>,
    /// TDLib 文件 ID（可选，用于 delete_file）。
    pub td_file_id: Option<i32>,
    /// 本地文件路径（可选，用于延迟删除）。
    pub local_path: Option<String>,
    /// 缓存最后一次错误信息。
    pub last_error: Option<String>,
    /// 活跃引用计数：当前被多少任务/子项引用。
    pub active_refs: i32,
    /// 引用计数降为 0 的时间。
    pub last_ref_zero_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// 进入删除队列后的到期删除时间。
    #[sea_orm(indexed)]
    pub delete_after: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// 缓存记录创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 缓存记录最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    /// 最近一次被使用的时间。
    pub last_used_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
