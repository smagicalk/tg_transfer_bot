// transfer_job 创建逻辑。
// 子项创建由 item 模块负责，这里只写入主任务记录。

use sea_orm::ActiveModelTrait;

use crate::db;
use crate::tgbot::transfer::types::{TransferBundle, TransferPlan, client_role_as_str};

use super::super::{JOB_STATUS_RUNNING, now_utc8};

/// 创建 transfer_job 主记录。
pub(in crate::tgbot::transfer) async fn create_job(
    plan: &TransferPlan,
    bundle: &TransferBundle,
) -> anyhow::Result<db::transfer_job::Model> {
    let db_conn = db::get_db().await?;
    let now = now_utc8();

    db::transfer_job::ActiveModel {
        request_chat_id: sea_orm::ActiveValue::Set(plan.request_chat_id),
        request_message_id: sea_orm::ActiveValue::Set(plan.request_message_id),
        source_link: sea_orm::ActiveValue::Set(plan.source_link.clone()),
        source_kind: sea_orm::ActiveValue::Set(plan.source_kind.as_str().to_owned()),
        source_client_role: sea_orm::ActiveValue::Set(
            client_role_as_str(bundle.source_client_role).to_owned(),
        ),
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
