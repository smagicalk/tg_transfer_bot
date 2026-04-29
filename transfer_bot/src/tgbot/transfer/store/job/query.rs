// transfer_job 查询函数。
// 这里只做读取，不改变任务状态，供 workflow 和命令层判断下一步动作。

use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;

use crate::db;

use super::super::{
    JOB_STATUS_CANCEL_FINALIZING, JOB_STATUS_CANCELLING, JOB_STATUS_PENDING, JOB_STATUS_RUNNING,
};

/// 根据请求消息查找是否已有任务（请求级幂等）。
pub(in crate::tgbot::transfer) async fn find_job_by_request(
    request_chat_id: i64,
    request_message_id: i64,
) -> anyhow::Result<Option<db::transfer_job::Model>> {
    let db_conn = db::get_db().await?;
    db::transfer_job::Entity::find()
        .filter(db::transfer_job::Column::RequestChatId.eq(request_chat_id))
        .filter(db::transfer_job::Column::RequestMessageId.eq(request_message_id))
        .one(db_conn)
        .await
        .map_err(Into::into)
}

/// 按 job_id 与请求 chat 查找任务。
/// 控制命令只允许管理当前请求聊天发起的任务，避免误操作其他聊天任务。
pub(in crate::tgbot::transfer) async fn find_job_for_request_chat(
    job_id: i64,
    request_chat_id: i64,
) -> anyhow::Result<Option<db::transfer_job::Model>> {
    let db_conn = db::get_db().await?;
    db::transfer_job::Entity::find_by_id(job_id)
        .filter(db::transfer_job::Column::RequestChatId.eq(request_chat_id))
        .one(db_conn)
        .await
        .map_err(Into::into)
}

/// 扫描待恢复任务：
/// 仅恢复 `pending/running`，已完成状态不再进入恢复流程。
pub(in crate::tgbot::transfer) async fn list_recoverable_jobs()
-> anyhow::Result<Vec<db::transfer_job::Model>> {
    let db_conn = db::get_db().await?;
    db::transfer_job::Entity::find()
        .filter(
            db::transfer_job::Column::Status
                .is_in([JOB_STATUS_PENDING.to_owned(), JOB_STATUS_RUNNING.to_owned()]),
        )
        .order_by_asc(db::transfer_job::Column::CreatedAt)
        .all(db_conn)
        .await
        .map_err(Into::into)
}

/// 扫描上次退出前已经请求停止、但尚未收尾的任务。
pub(in crate::tgbot::transfer) async fn list_cancelling_jobs()
-> anyhow::Result<Vec<db::transfer_job::Model>> {
    let db_conn = db::get_db().await?;
    db::transfer_job::Entity::find()
        .filter(db::transfer_job::Column::Status.is_in([
            JOB_STATUS_CANCELLING.to_owned(),
            JOB_STATUS_CANCEL_FINALIZING.to_owned(),
        ]))
        .order_by_asc(db::transfer_job::Column::UpdatedAt)
        .all(db_conn)
        .await
        .map_err(Into::into)
}

/// 读取任务当前状态。
pub(in crate::tgbot::transfer) async fn get_job_status(
    job_id: i64,
) -> anyhow::Result<Option<String>> {
    let db_conn = db::get_db().await?;
    Ok(db::transfer_job::Entity::find_by_id(job_id)
        .one(db_conn)
        .await?
        .map(|job| job.status))
}
