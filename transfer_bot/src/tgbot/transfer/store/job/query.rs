// transfer_job 查询函数。
// 这里只做读取，不改变任务状态，供 workflow 和命令层判断下一步动作。

use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;

use crate::db;

use super::super::{
    JOB_STATUS_CANCEL_FINALIZING, JOB_STATUS_CANCELLING, JOB_STATUS_PENDING, JOB_STATUS_RUNNING,
};

/// 根据请求会话与消息 ID 查找是否已有任务（请求级幂等排重）。
///
/// 避免用户连点或网络重发导致为同一条命令消息创建多个后台任务。
///
/// # 参数
/// - `request_chat_id`: 发起请求的 Telegram 聊天 ID。
/// - `request_message_id`: 发起请求的 Telegram 消息 ID。
///
/// # 返回值
/// - 已存在的 `transfer_job::Model`（若有）。
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

/// 按主键 ID 查询任务模型。
///
/// # 参数
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - `Some(Model)` 对应的任务模型，或 `None`（不存在）。
pub(in crate::tgbot::transfer) async fn find_job(
    job_id: i64,
) -> anyhow::Result<Option<db::transfer_job::Model>> {
    let db_conn = db::get_db().await?;
    db::transfer_job::Entity::find_by_id(job_id)
        .one(db_conn)
        .await
        .map_err(Into::into)
}

/// 扫描可恢复执行的任务列表。
///
/// 条件为处于 `pending` 或 `running` 状态，按创建时间升序排列。
/// 用于服务重启后自动接续执行中断的任务。
///
/// # 返回值
/// - 待恢复的任务模型列表。
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

/// 扫描上次退出前已经请求停止、但尚未完成收尾的任务。
///
/// 条件为处于 `cancelling` 或 `cancel_finalizing` 状态，用于服务重启时完成清理。
///
/// # 返回值
/// - 待收尾取消的任务列表。
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

/// 轻量读取任务当前状态。
///
/// 仅投影 `status` 字段，避免全量读出无关数据字段，供控制流程频繁轮询判断。
///
/// # 参数
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - 任务当前的状态字符串（如 "running"、"paused"、"cancelling" 等）。
pub(in crate::tgbot::transfer) async fn get_job_status(
    job_id: i64,
) -> anyhow::Result<Option<String>> {
    let db_conn = db::get_db().await?;
    // 控制流程只需要 status，按列投影避免把 transfer_job 全字段读出来。
    Ok(db::transfer_job::Entity::find()
        .select_only()
        .column(db::transfer_job::Column::Status)
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .into_tuple::<String>()
        .one(db_conn)
        .await?)
}
