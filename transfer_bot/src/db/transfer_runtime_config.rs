//! 转存调度与 GC 运行时控制配置数据表实体模型。
//!
//! 持久化允许在运行时动态调整的转存控制参数。
//! 表中固定只保存单行记录（`id = 1`），使得管理员通过 Bot 命令热更新的参数能够跨进程重启保存。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// 转存运行时动态控制参数的数据库持久化模型。
#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_runtime_config")]
pub struct Model {
    /// 单行配置主键，固定为 1。
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    /// 后台转存任务并发数上限。
    pub job_concurrency: i64,
    /// 文件引用归零后延迟物理删除的缓冲时间（分钟）。
    pub file_delete_delay_minutes: i64,
    /// 本地文件垃圾回收（GC）巡检扫描周期（秒）。
    pub file_gc_interval_seconds: i64,
    /// 转存进度卡片消息编辑刷新最小间隔时间（秒）。
    pub progress_edit_interval_seconds: i64,
    /// `/downloads` 列表命令的默认单页展示条目数。
    pub downloads_default_page_size: i64,
    /// 交互式菜单等待用户输入文本或按钮操作的超时时间（秒）。
    pub menu_input_timeout_seconds: i64,
    /// 配置记录首次创建入库时间戳。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 配置记录最近一次修改更新的时间戳。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
