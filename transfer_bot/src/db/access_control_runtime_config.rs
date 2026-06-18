// access_control_runtime_config 实体：
// 保存访问控制的布尔开关与基础运行规则。

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "access_control_runtime_config")]
pub struct Model {
    /// 单行配置主键，固定为 1。
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    /// 是否允许任意私聊用户作为普通用户使用。
    pub allow_all_private_users: bool,
    /// 配置记录创建时间。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 配置记录最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ActiveModelBehavior for ActiveModel {}
