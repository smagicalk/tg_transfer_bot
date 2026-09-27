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
use super::query::find_job;

/// 将任务状态原子标记为 running（开始或恢复执行前触发）。
///
/// 校验规则：只允许从 `pending` 或原 `running` 状态流转到 `running`；
/// 若已被暂停（paused）、停止（cancelling）或已终态，则拒绝更新。
///
/// # 参数
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - `true`: 成功置为 running，调用方可安全推进后续工作流；
/// - `false`: 任务状态已被其它并发操作抢占，调用方必须停止执行。
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

/// 将任务标记为暂停（paused）。
///
/// 允许从 `pending`、`running`、`paused` 迁移；处于 cancelling 或已终态的任务不可暂停。
///
/// # 参数
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - 更新后的 `transfer_job::Model` 数据库模型。
pub(in crate::tgbot::transfer) async fn pause_job(
    job_id: i64,
) -> anyhow::Result<db::transfer_job::Model> {
    let db_conn = db::get_db().await?;
    let job = find_job(job_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found: {job_id}"))?;

    match job.status.as_str() {
        JOB_STATUS_PENDING | JOB_STATUS_RUNNING | JOB_STATUS_PAUSED => {}
        JOB_STATUS_CANCELLING | JOB_STATUS_CANCEL_FINALIZING => {
            anyhow::bail!("job is cancelling: {job_id}")
        }
        status if is_finished_job_status(status) => {
            anyhow::bail!("job already finished: {status}")
        }
        status => anyhow::bail!("job status doesn't support pause: {status}"),
    }

    let update = db::transfer_job::Entity::update_many()
        .col_expr(
            db::transfer_job::Column::Status,
            Expr::value(JOB_STATUS_PAUSED),
        )
        .col_expr(db::transfer_job::Column::UpdatedAt, Expr::value(now_utc8()))
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .filter(db::transfer_job::Column::Status.is_in([
            JOB_STATUS_PENDING.to_owned(),
            JOB_STATUS_RUNNING.to_owned(),
            JOB_STATUS_PAUSED.to_owned(),
        ]));
    let rs = update.exec(db_conn).await?;

    if rs.rows_affected == 0 {
        anyhow::bail!("job status changed before pause: {job_id}");
    }

    find_job(job_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found after pause: {job_id}"))
}

/// 唤醒未完成任务（恢复执行）。
///
/// 状态迁移语义：
/// - `paused`: 原子更新为 `pending`，重回调度队列；
/// - `pending` / `running`: 任务本身已在可执行状态，直接返回现有模型（用于后台协程中断后的手动补触发）；
/// - `finished` / `cancelling`: 拒绝恢复，避免数据污染。
///
/// # 参数
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - 唤醒后的 `transfer_job::Model`。
pub(in crate::tgbot::transfer) async fn wake_job(
    job_id: i64,
) -> anyhow::Result<db::transfer_job::Model> {
    let db_conn = db::get_db().await?;
    let job = find_job(job_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found: {job_id}"))?;

    match job.status.as_str() {
        JOB_STATUS_PAUSED => {}
        JOB_STATUS_PENDING | JOB_STATUS_RUNNING => return Ok(job),
        JOB_STATUS_CANCELLING | JOB_STATUS_CANCEL_FINALIZING => {
            anyhow::bail!("job is cancelling: {job_id}")
        }
        status if is_finished_job_status(status) => {
            anyhow::bail!("job already finished: {status}")
        }
        status => anyhow::bail!("job status doesn't support wake: {status}"),
    }

    let update = db::transfer_job::Entity::update_many()
        .col_expr(
            db::transfer_job::Column::Status,
            Expr::value(JOB_STATUS_PENDING),
        )
        .col_expr(db::transfer_job::Column::UpdatedAt, Expr::value(now_utc8()))
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .filter(db::transfer_job::Column::Status.eq(JOB_STATUS_PAUSED));
    let rs = update.exec(db_conn).await?;

    if rs.rows_affected == 0 {
        anyhow::bail!("job status changed before wake: {job_id}");
    }

    find_job(job_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found after wake: {job_id}"))
}

/// 用户请求停止任务（协作式取消第一步）。
///
/// 将状态变更为 `cancelling`。后台正在执行的转存循环在每次循环或网络检查点
/// 会感知到该状态，并在安全点调用 `cancel_job_now` 完成文件清理与终态收尾。
///
/// # 参数
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - 标记为 cancelling 或最新状态的模型对象。
pub(in crate::tgbot::transfer) async fn request_cancel_job(
    job_id: i64,
) -> anyhow::Result<db::transfer_job::Model> {
    let db_conn = db::get_db().await?;
    let job = find_job(job_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found: {job_id}"))?;

    if job.status == JOB_STATUS_CANCEL_FINALIZING {
        return Ok(job);
    }

    match job.status.as_str() {
        JOB_STATUS_PENDING | JOB_STATUS_RUNNING | JOB_STATUS_PAUSED | JOB_STATUS_CANCELLING => {}
        status if is_finished_job_status(status) => {
            anyhow::bail!("job already finished: {status}")
        }
        status => anyhow::bail!("job status doesn't support stop: {status}"),
    }

    let update = db::transfer_job::Entity::update_many()
        .col_expr(
            db::transfer_job::Column::Status,
            Expr::value(JOB_STATUS_CANCELLING),
        )
        .col_expr(db::transfer_job::Column::UpdatedAt, Expr::value(now_utc8()))
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .filter(db::transfer_job::Column::Status.is_in([
            JOB_STATUS_PENDING.to_owned(),
            JOB_STATUS_RUNNING.to_owned(),
            JOB_STATUS_PAUSED.to_owned(),
            JOB_STATUS_CANCELLING.to_owned(),
        ]));
    let rs = update.exec(db_conn).await?;

    if rs.rows_affected == 0 {
        let current = find_job(job_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("job not found after stop conflict: {job_id}"))?;
        if matches!(
            current.status.as_str(),
            JOB_STATUS_CANCEL_FINALIZING | JOB_STATUS_CANCELLED
        ) {
            return Ok(current);
        }
        anyhow::bail!("job status changed before stop: {job_id}");
    }

    find_job(job_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found after stop: {job_id}"))
}
