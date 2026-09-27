//! 默认目标频道配置数据表实体模型。
//!
//! 保存用户通过 `/targets set_default` 设置的全局默认转存频道/群组。
//! 表中固定只保存单行记录（`id = 1`）。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// 默认目标频道配置持久化模型。
#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_target_config")]
pub struct Model {
    /// 单行配置主键，固定为 1。
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    /// 全局默认目标频道或群组的 Telegram Chat ID；0 表示未配置默认目标。
    pub default_chat_id: i64,
    /// 配置记录首次创建时间戳。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 配置记录最近更新时间戳。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
