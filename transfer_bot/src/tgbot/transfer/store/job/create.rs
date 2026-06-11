// transfer_job 创建逻辑。
// 子项创建由 item 模块负责，这里只写入主任务记录。

use sea_orm::ActiveModelTrait;

use crate::config::ActorRole;
use crate::db;
use crate::tgbot::transfer::types::{TransferBundle, TransferPlan, client_role_as_str};

use super::super::{JOB_STATUS_RUNNING, now_utc8};

/// 创建 transfer_job 主记录。
pub(in crate::tgbot::transfer) async fn create_job(
    plan: &TransferPlan,
    bundle: &TransferBundle,
    billing: CreateJobBilling,
) -> anyhow::Result<db::transfer_job::Model> {
    let db_conn = db::get_db().await?;
    let now = now_utc8();

    db::transfer_job::ActiveModel {
        request_chat_id: sea_orm::ActiveValue::Set(plan.request_chat_id),
        request_message_id: sea_orm::ActiveValue::Set(plan.request_message_id),
        owner_user_id: sea_orm::ActiveValue::Set(plan.actor.user_id),
        source_link: sea_orm::ActiveValue::Set(plan.source_link.clone()),
        source_kind: sea_orm::ActiveValue::Set(plan.source_kind.as_str().to_owned()),
        source_client_role: sea_orm::ActiveValue::Set(
            client_role_as_str(bundle.source_client_role).to_owned(),
        ),
        allow_user_fallback: sea_orm::ActiveValue::Set(plan.allow_user_fallback),
        source_chat_id: sea_orm::ActiveValue::Set(bundle.source_chat_id),
        source_message_id: sea_orm::ActiveValue::Set(bundle.source_message_id),
        source_album_id: sea_orm::ActiveValue::Set(bundle.source_album_id),
        target_chat_id: sea_orm::ActiveValue::Set(plan.target_chat_id),
        result_message_id: sea_orm::ActiveValue::Set(None),
        result_message_link: sea_orm::ActiveValue::Set(None),
        status: sea_orm::ActiveValue::Set(JOB_STATUS_RUNNING.to_owned()),
        total_items: sea_orm::ActiveValue::Set(bundle.messages.len() as i32),
        done_items: sea_orm::ActiveValue::Set(0),
        failed_items: sea_orm::ActiveValue::Set(0),
        retry_count: sea_orm::ActiveValue::Set(0),
        cost_points: sea_orm::ActiveValue::Set(billing.cost_points),
        charged_points: sea_orm::ActiveValue::Set(billing.charged_points),
        billing_status: sea_orm::ActiveValue::Set(billing.status.to_owned()),
        last_error: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        finished_at: sea_orm::ActiveValue::Set(None),
        ..Default::default()
    }
    .insert(db_conn)
    .await
    .map_err(Into::into)
}

/// 创建任务时写入的计费摘要。
///
/// admin 与免费模式写入 free/0；普通用户扣费成功后写入 charged/扣费值。
#[derive(Debug, Clone, Copy)]
pub(in crate::tgbot::transfer) struct CreateJobBilling {
    pub cost_points: i64,
    pub charged_points: i64,
    pub status: &'static str,
}

impl CreateJobBilling {
    /// 根据发起人角色和任务成本生成数据库字段。
    pub(in crate::tgbot::transfer) fn new(actor_role: ActorRole, cost_points: i64) -> Self {
        if actor_role.is_admin() || cost_points <= 0 {
            Self {
                cost_points: 0,
                charged_points: 0,
                status: "free",
            }
        } else {
            Self {
                cost_points,
                charged_points: cost_points,
                status: "charged",
            }
        }
    }
}
