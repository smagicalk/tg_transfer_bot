//! 本地文件缓存元数据与引用计数数据表实体模型。
//!
//! 以 `(owner_client_role, file_key)` 形成复合主键，用于跨转存任务下载防重、
//! 共享已下载本地文件、追踪活跃引用计数以及排队延迟垃圾回收（GC）。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// 本地文件缓存与生命周期追踪数据模型。
#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "file_cache")]
pub struct Model {
    /// 文件所属 TDLib 客户端角色：`bot` 或 `user`。
    #[sea_orm(primary_key, auto_increment = false)]
    pub owner_client_role: String,
    /// 文件全局唯一识别键（用于跨任务识别相同文件并去重）。
    #[sea_orm(primary_key, auto_increment = false)]
    pub file_key: String,
    /// 缓存状态：如 `downloading`（下载中）、`ready`（已就绪）、`failed`（失败）等。
    #[sea_orm(indexed)]
    pub status: String,
    /// 文件总大小（字节，已知时记录）。
    pub size_bytes: Option<i64>,
    /// TDLib 内部分配的当前 File ID（用于向 TDLib 发起删除等调用）。
    pub td_file_id: Option<i32>,
    /// 本地落盘存储文件的绝对路径。
    pub local_path: Option<String>,
    /// 最近一次下载或处理失败的错误原因描述。
    pub last_error: Option<String>,
    /// 活跃引用计数：当前正在引用此文件的任务条目总数。
    pub active_refs: i32,
    /// 引用计数降为 0 的时间戳（开始进入延迟删除倒计时）。
    pub last_ref_zero_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// 延迟清理到期时间戳；后台 GC 扫描时超过该时间的文件将被物理删除。
    #[sea_orm(indexed)]
    pub delete_after: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// 缓存记录创建时间戳。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 缓存记录最后更新时间戳。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    /// 最近一次被任务读取或引用的时间戳。
    pub last_used_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
