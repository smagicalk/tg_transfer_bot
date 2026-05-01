// transfer_job 完成状态写入。
// 这里统一处理主任务终态、子项批量更新和 file_cache 引用释放。

use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::TransactionTrait;

use crate::db;

use super::super::file_cache::release_job_file_refs_on_conn;
use super::super::item::set_item_status_on_conn;
use super::super::{
    FinishJobSummary, JOB_STATUS_CANCEL_FINALIZING, JOB_STATUS_CANCELLED, JOB_STATUS_CANCELLING,
    JOB_STATUS_FAILED, JOB_STATUS_PARTIAL, JOB_STATUS_PAUSED, JOB_STATUS_PENDING,
    JOB_STATUS_RUNNING, JOB_STATUS_SUCCESS, is_finished_job_status, now_utc8,
};

/// 更新已完成任务的结果链接。
///
/// 旧版本可能保存过不可点击的 `tg://openmessage` 或纯定位字符串；当重复转存
/// 或 `/lookup` 成功用 TDLib 重新生成入口链接后，在这里写回数据库，后续命中
/// 同一 source_link + target_chat_id 时可以直接返回可点击链接。
pub(in crate::tgbot::transfer) async fn update_result_message_link(
    job_id: i64,
    result_message_link: String,
) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    db::transfer_job::Entity::update_many()
        .set(db::transfer_job::ActiveModel {
            result_message_link: sea_orm::ActiveValue::Set(Some(result_message_link)),
            updated_at: sea_orm::ActiveValue::Set(now_utc8()),
            ..Default::default()
        })
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .exec(db_conn)
        .await?;
    Ok(())
}

/// 完成任务并写回汇总状态。
///
/// 返回 true 表示终态写入成功；返回 false 表示任务已被停止/终止状态抢先更新，
/// 调用方应转入控制状态处理，避免覆盖用户的停止请求。
pub(in crate::tgbot::transfer) async fn finish_job(
    job: db::transfer_job::Model,
    ok_count: i32,
    fail_count: i32,
    last_error: Option<String>,
    result_message_id: Option<i64>,
    result_message_link: Option<String>,
    delay_hours: i64,
) -> anyhow::Result<bool> {
    let summary = FinishJobSummary {
        ok_count,
        fail_count,
        last_error,
        result_message_id,
        result_message_link,
        delay_hours,
    };
    finish_job_with_allowed_statuses(
        job,
        summary,
        Vec::new(),
        &[JOB_STATUS_PENDING, JOB_STATUS_RUNNING],
    )
    .await
}

/// 完成任务，同时在同一事务内更新一批子项状态并释放文件引用。
///
/// 用于“准备失败/上传失败”等路径，避免主任务已终态但子项或引用仍未同步。
pub(in crate::tgbot::transfer) async fn finish_job_with_item_statuses(
    job: db::transfer_job::Model,
    summary: FinishJobSummary,
    item_updates: Vec<(i64, String, Option<String>)>,
) -> anyhow::Result<bool> {
    finish_job_with_allowed_statuses(
        job,
        summary,
        item_updates,
        &[JOB_STATUS_PENDING, JOB_STATUS_RUNNING],
    )
    .await
}

/// 上传已经成功后的终态写入。
///
/// 进入这里说明目标消息已经发出，暂停/停止都无法再撤回上传结果，
/// 因此允许从 paused/cancelling 收敛为成功，避免数据库隐藏真实已转存结果。
#[cfg(test)]
pub(in crate::tgbot::transfer) async fn finish_uploaded_job(
    job: db::transfer_job::Model,
    ok_count: i32,
    fail_count: i32,
    last_error: Option<String>,
    result_message_id: Option<i64>,
    result_message_link: Option<String>,
    delay_hours: i64,
) -> anyhow::Result<bool> {
    let summary = FinishJobSummary {
        ok_count,
        fail_count,
        last_error,
        result_message_id,
        result_message_link,
        delay_hours,
    };
    finish_job_with_allowed_statuses(
        job,
        summary,
        Vec::new(),
        &[
            JOB_STATUS_PENDING,
            JOB_STATUS_RUNNING,
            JOB_STATUS_PAUSED,
            JOB_STATUS_CANCELLING,
        ],
    )
    .await
}

/// 上传成功后完成任务，并在同一事务内把已上传子项标记为成功。
pub(in crate::tgbot::transfer) async fn finish_uploaded_job_with_item_statuses(
    job: db::transfer_job::Model,
    summary: FinishJobSummary,
    item_updates: Vec<(i64, String, Option<String>)>,
) -> anyhow::Result<bool> {
    finish_job_with_allowed_statuses(
        job,
        summary,
        item_updates,
        &[
            JOB_STATUS_PENDING,
            JOB_STATUS_RUNNING,
            JOB_STATUS_PAUSED,
            JOB_STATUS_CANCELLING,
        ],
    )
    .await
}

/// 按允许的源状态集合完成任务。
///
/// 普通 finish 只允许 pending/running；上传成功后的 finish 允许更多控制态，
/// 因为目标消息已经实际发出，数据库需要记录真实结果。
async fn finish_job_with_allowed_statuses(
    job: db::transfer_job::Model,
    summary: FinishJobSummary,
    item_updates: Vec<(i64, String, Option<String>)>,
    allowed_statuses: &[&str],
) -> anyhow::Result<bool> {
    let db_conn = db::get_db().await?;
    let txn = db_conn.begin().await?;
    let final_status = if summary.fail_count == 0 {
        JOB_STATUS_SUCCESS
    } else if summary.ok_count == 0 {
        JOB_STATUS_FAILED
    } else {
        JOB_STATUS_PARTIAL
    };

    let active = db::transfer_job::ActiveModel {
        done_items: sea_orm::ActiveValue::Set(summary.ok_count),
        failed_items: sea_orm::ActiveValue::Set(summary.fail_count),
        result_message_id: sea_orm::ActiveValue::Set(summary.result_message_id),
        result_message_link: sea_orm::ActiveValue::Set(summary.result_message_link),
        status: sea_orm::ActiveValue::Set(final_status.to_owned()),
        last_error: sea_orm::ActiveValue::Set(summary.last_error),
        updated_at: sea_orm::ActiveValue::Set(now_utc8()),
        finished_at: sea_orm::ActiveValue::Set(Some(now_utc8())),
        ..Default::default()
    };

    let rs = db::transfer_job::Entity::update_many()
        .set(active)
        .filter(db::transfer_job::Column::Id.eq(job.id))
        .filter(
            db::transfer_job::Column::Status
                .is_in(allowed_statuses.iter().map(|status| (*status).to_owned())),
        )
        .exec(&txn)
        .await?;

    if rs.rows_affected > 0 {
        for (item_id, status, error_message) in item_updates {
            set_item_status_on_conn(&txn, item_id, &status, error_message).await?;
        }
        release_job_file_refs_on_conn(&txn, job.id, summary.delay_hours).await?;
        txn.commit().await?;
        return Ok(true);
    }

    let Some(current) = db::transfer_job::Entity::find_by_id(job.id)
        .one(&txn)
        .await?
    else {
        txn.rollback().await?;
        anyhow::bail!("job not found during finish: {}", job.id);
    };

    if current.status == JOB_STATUS_PAUSED
        || current.status == JOB_STATUS_CANCELLING
        || current.status == JOB_STATUS_CANCEL_FINALIZING
        || current.status == JOB_STATUS_CANCELLED
        || is_finished_job_status(&current.status)
    {
        txn.rollback().await?;
        return Ok(false);
    }

    txn.rollback().await?;
    anyhow::bail!(
        "job status doesn't support finish, job_id={}, status={}",
        job.id,
        current.status
    )
}
