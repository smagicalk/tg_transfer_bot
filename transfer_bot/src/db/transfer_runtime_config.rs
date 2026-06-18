// transfer_runtime_config 实体：
// 持久化允许在运行时动态调整的转存参数。
// 这里固定只保存一行 `id = 1`，避免把启动配置和运行参数混进同一个 JSON 文件。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_runtime_config")]
pub struct Model {
    /// 单行配置主键，固定为 1。
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    /// 后台任务并发数。
    pub job_concurrency: i64,
    /// 文件引用归零后延迟删除分钟数。
    pub file_delete_delay_minutes: i64,
    /// 文件 GC 扫描间隔秒数。
    pub file_gc_interval_seconds: i64,
    /// 进度消息编辑间隔秒数。
    pub progress_edit_interval_seconds: i64,
    /// 下载列表默认分页大小。
    pub downloads_default_page_size: i64,
    /// 菜单输入超时秒数。
    pub menu_input_timeout_seconds: i64,
    /// 配置记录创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 配置记录最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
