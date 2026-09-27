//! 目标频道快捷别名映射数据表实体模型。
//!
//! 保存用户自定义的短别名与实际 Telegram 频道/群组 Chat ID 之间的映射关系，
//! 允许用户在 `/transfer <link> <alias>` 命令中直接指定别名而无需记忆复杂负数 ID。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// 快捷别名与目标频道 Chat ID 映射持久化模型。
#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_target_alias")]
pub struct Model {
    /// 快捷别名字符串主键（例如 "anime"、"movies"）。
    #[sea_orm(primary_key, auto_increment = false)]
    pub alias: String,
    /// 别名所指向的目标频道或群组 Telegram Chat ID。
    pub target_chat_id: i64,
    /// 别名映射首次创建时间戳。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 别名映射最近修改时间戳。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
