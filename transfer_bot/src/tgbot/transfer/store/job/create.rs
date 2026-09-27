// transfer_job 创建逻辑。
// 子项创建由 item 模块负责，这里只写入主任务记录。

use sea_orm::ActiveModelTrait;

use crate::db;
use crate::tgbot::transfer::types::{TransferBundle, TransferPlan, client_role_as_str};

use super::super::{JOB_STATUS_RUNNING, now_utc8};

/// 创建 `transfer_job` 主记录。
///
/// 将转存规划（`TransferPlan`）与爬虫解析出的消息包（`TransferBundle`）持久化到数据库中。
/// 初始状态设为 `running`，条目总数设置为 `bundle.messages.len()`，各进度计数归零。
///
/// # 参数
/// - `plan`: 用户发起转存请求的规划参数（发起人、来源链接、目标群组等）。
/// - `bundle`: 抓取到的消息实体与客户端角色上下文。
///
/// # 返回值
/// - 成功写入的 `transfer_job::Model` 数据库模型。
pub(in crate::tgbot::transfer) async fn create_job(
    plan: &TransferPlan,
    bundle: &TransferBundle,
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
