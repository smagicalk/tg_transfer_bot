// transfer_job 控制状态迁移。
// 这里处理用户手动 pause/resume/stop 请求，但不做最终引用释放。

use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::sea_query::Expr;

use crate::db;

use super::super::{
    JOB_STATUS_CANCEL_FINALIZING, JOB_STATUS_CANCELLED, JOB_STATUS_CANCELLING, JOB_STATUS_PAUSED,
    JOB_STATUS_PENDING, JOB_STATUS_RUNNING, is_finished_job_status, now_utc8,
};
use super::query::find_job_for_request_chat;

/// 将任务状态标记为 running（恢复前触发）。
///
/// 返回 false 表示任务已被暂停、停止或完成状态抢先占用，调用方不能继续创建子项或下载文件。
pub(in crate::tgbot::transfer) async fn mark_job_running(job_id: i64) -> anyhow::Result<bool> {
    let db_conn = db::get_db().await?;
    // 只允许 pending/running 进入 running，避免恢复流程覆盖暂停或停止请求。
    let rs = db::transfer_job::Entity::update_many()
        .col_expr(
            db::transfer_job::Column::Status,
            Expr::value(JOB_STATUS_RUNNING),
        )
        .col_expr(db::transfer_job::Column::UpdatedAt, Expr::value(now_utc8()))
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .filter(
            db::transfer_job::Column::Status
                .is_in([JOB_STATUS_PENDING.to_owned(), JOB_STATUS_RUNNING.to_owned()]),
        )
        .exec(db_conn)
        .await?;
    Ok(rs.rows_affected > 0)
}

/// 将任务标记为暂停。
pub(in crate::tgbot::transfer) async fn pause_job(
    job_id: i64,
    request_chat_id: i64,
) -> anyhow::Result<db::transfer_job::Model> {
    let db_conn = db::get_db().await?;
    let job = find_job_for_request_chat(job_id, request_chat_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found: {}", job_id))?;

    match job.status.as_str() {
        JOB_STATUS_PENDING | JOB_STATUS_RUNNING | JOB_STATUS_PAUSED => {}
        JOB_STATUS_CANCELLING | JOB_STATUS_CANCEL_FINALIZING => {
            anyhow::bail!("job is cancelling: {}", job_id)
        }
        status if is_finished_job_status(status) => {
            anyhow::bail!("job already finished: {}", status)
        }
        status => anyhow::bail!("job status doesn't support pause: {}", status),
    }

    let rs = db::transfer_job::Entity::update_many()
        .col_expr(
            db::transfer_job::Column::Status,
            Expr::value(JOB_STATUS_PAUSED),
        )
        .col_expr(db::transfer_job::Column::UpdatedAt, Expr::value(now_utc8()))
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .filter(db::transfer_job::Column::RequestChatId.eq(request_chat_id))
        .filter(db::transfer_job::Column::Status.is_in([
            JOB_STATUS_PENDING.to_owned(),
            JOB_STATUS_RUNNING.to_owned(),
            JOB_STATUS_PAUSED.to_owned(),
        ]))
        .exec(db_conn)
        .await?;

    if rs.rows_affected == 0 {
        anyhow::bail!("job status changed before pause: {}", job_id);
    }

    find_job_for_request_chat(job_id, request_chat_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found after pause: {}", job_id))
}

/// 唤醒未完成任务，随后由命令层决定是否重新派发后台执行。
///
/// 语义：
/// - paused：改回 pending，等待后台继续处理。
/// - pending/running：任务本身可继续执行，直接返回，用于处理后台 task 丢失后的手动补派发。
/// - finished/cancelling：拒绝恢复，避免重复释放引用或重复上传。
pub(in crate::tgbot::transfer) async fn wake_job(
    job_id: i64,
    request_chat_id: i64,
) -> anyhow::Result<db::transfer_job::Model> {
    let db_conn = db::get_db().await?;
    let job = find_job_for_request_chat(job_id, request_chat_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found: {}", job_id))?;

    match job.status.as_str() {
        JOB_STATUS_PAUSED => {}
        JOB_STATUS_PENDING | JOB_STATUS_RUNNING => return Ok(job),
        JOB_STATUS_CANCELLING | JOB_STATUS_CANCEL_FINALIZING => {
            anyhow::bail!("job is cancelling: {}", job_id)
        }
        status if is_finished_job_status(status) => {
            anyhow::bail!("job already finished: {}", status)
        }
        status => anyhow::bail!("job status doesn't support wake: {}", status),
    }

    let rs = db::transfer_job::Entity::update_many()
        .col_expr(
            db::transfer_job::Column::Status,
            Expr::value(JOB_STATUS_PENDING),
        )
        .col_expr(db::transfer_job::Column::UpdatedAt, Expr::value(now_utc8()))
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .filter(db::transfer_job::Column::RequestChatId.eq(request_chat_id))
        .filter(db::transfer_job::Column::Status.eq(JOB_STATUS_PAUSED))
        .exec(db_conn)
        .await?;

    if rs.rows_affected == 0 {
        anyhow::bail!("job status changed before wake: {}", job_id);
    }

    find_job_for_request_chat(job_id, request_chat_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found after wake: {}", job_id))
}

/// 将运行中的任务标记为 cancelling。
/// 后台工作流会在下一个安全点调用 `cancel_job_now` 完成收尾。
pub(in crate::tgbot::transfer) async fn request_cancel_job(
    job_id: i64,
    request_chat_id: i64,
) -> anyhow::Result<db::transfer_job::Model> {
    let db_conn = db::get_db().await?;
    let job = find_job_for_request_chat(job_id, request_chat_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found: {}", job_id))?;

    if job.status == JOB_STATUS_CANCEL_FINALIZING {
        return Ok(job);
    }

    match job.status.as_str() {
        JOB_STATUS_PENDING | JOB_STATUS_RUNNING | JOB_STATUS_PAUSED | JOB_STATUS_CANCELLING => {}
        status if is_finished_job_status(status) => {
            anyhow::bail!("job already finished: {}", status)
        }
        status => anyhow::bail!("job status doesn't support stop: {}", status),
    }

    let rs = db::transfer_job::Entity::update_many()
        .col_expr(
            db::transfer_job::Column::Status,
            Expr::value(JOB_STATUS_CANCELLING),
        )
        .col_expr(db::transfer_job::Column::UpdatedAt, Expr::value(now_utc8()))
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .filter(db::transfer_job::Column::RequestChatId.eq(request_chat_id))
        .filter(db::transfer_job::Column::Status.is_in([
            JOB_STATUS_PENDING.to_owned(),
            JOB_STATUS_RUNNING.to_owned(),
            JOB_STATUS_PAUSED.to_owned(),
            JOB_STATUS_CANCELLING.to_owned(),
        ]))
        .exec(db_conn)
        .await?;

    if rs.rows_affected == 0 {
        let current = find_job_for_request_chat(job_id, request_chat_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("job not found after stop conflict: {}", job_id))?;
        if matches!(
            current.status.as_str(),
            JOB_STATUS_CANCEL_FINALIZING | JOB_STATUS_CANCELLED
        ) {
            return Ok(current);
        }
        anyhow::bail!("job status changed before stop: {}", job_id);
    }

    find_job_for_request_chat(job_id, request_chat_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found after stop: {}", job_id))
}
