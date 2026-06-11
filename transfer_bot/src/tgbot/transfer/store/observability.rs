// 运行观测只读查询。
// `/health` 和 `/cache` 只读取这些快照，不修改任务、子项或 file_cache 状态。

use std::collections::BTreeMap;

use sea_orm::{EntityTrait, QueryOrder, QuerySelect};

use crate::db;

use super::{
    FILE_CACHE_STATUS_DELETE_FAILED, FileCacheSnapshot, FileCacheStatusSummary,
    ITEM_STATUS_PREPARING, ITEM_STATUS_UPLOADING, JOB_STATUS_CANCEL_FINALIZING,
    JOB_STATUS_CANCELLED, JOB_STATUS_CANCELLING, JOB_STATUS_FAILED, JOB_STATUS_PARTIAL,
    JOB_STATUS_PAUSED, JOB_STATUS_PENDING, JOB_STATUS_RUNNING, JOB_STATUS_SUCCESS,
    TransferHealthSnapshot, now_utc8,
};

/// 读取转存系统健康快照。
pub(in crate::tgbot::transfer) async fn list_transfer_health_snapshot(
    app_context: &crate::app_context::AppContext,
) -> anyhow::Result<TransferHealthSnapshot> {
    let db_conn = db::get_db().await?;
    let now = now_utc8();
    let runtime_config = app_context.transfer_runtime.runtime_config();

    let job_statuses = db::transfer_job::Entity::find()
        .select_only()
        .column(db::transfer_job::Column::Status)
        .into_tuple::<String>()
        .all(db_conn)
        .await?;
    let item_statuses = db::transfer_item::Entity::find()
        .select_only()
        .column(db::transfer_item::Column::Status)
        .into_tuple::<String>()
        .all(db_conn)
        .await?;
    let file_cache_rows = db::file_cache::Entity::find()
        .select_only()
        .column(db::file_cache::Column::Status)
        .column(db::file_cache::Column::ActiveRefs)
        .column(db::file_cache::Column::DeleteAfter)
        .into_tuple::<(String, i32, Option<chrono::DateTime<chrono::FixedOffset>>)>()
        .all(db_conn)
        .await?;

    let active_jobs = count_statuses(
        &job_statuses,
        &[
            JOB_STATUS_PENDING,
            JOB_STATUS_RUNNING,
            JOB_STATUS_PAUSED,
            JOB_STATUS_CANCELLING,
            JOB_STATUS_CANCEL_FINALIZING,
        ],
    );
    let recoverable_jobs = count_statuses(&job_statuses, &[JOB_STATUS_PENDING, JOB_STATUS_RUNNING]);
    let cancelling_jobs = count_statuses(
        &job_statuses,
        &[JOB_STATUS_CANCELLING, JOB_STATUS_CANCEL_FINALIZING],
    );
    let failed_jobs = count_statuses(&job_statuses, &[JOB_STATUS_FAILED, JOB_STATUS_PARTIAL]);
    let file_cache_active_rows = file_cache_rows
        .iter()
        .filter(|(_, active_refs, _)| *active_refs > 0)
        .count() as i64;
    let file_cache_due_rows = file_cache_rows
        .iter()
        .filter(|(_, active_refs, delete_after)| {
            *active_refs == 0 && delete_after.is_some_and(|delete_after| delete_after <= now)
        })
        .count() as i64;
    let file_cache_failed_rows = file_cache_rows
        .iter()
        .filter(|(status, _, _)| status.as_str() == FILE_CACHE_STATUS_DELETE_FAILED)
        .count() as i64;

    Ok(TransferHealthSnapshot {
        total_jobs: job_statuses.len() as i64,
        active_jobs,
        success_jobs: count_statuses(&job_statuses, &[JOB_STATUS_SUCCESS]),
        failed_jobs,
        cancelled_jobs: count_statuses(&job_statuses, &[JOB_STATUS_CANCELLED]),
        total_items: item_statuses.len() as i64,
        preparing_items: count_statuses(&item_statuses, &[ITEM_STATUS_PREPARING]),
        uploading_items: count_statuses(&item_statuses, &[ITEM_STATUS_UPLOADING]),
        file_cache_rows: file_cache_rows.len() as i64,
        file_cache_active_rows,
        file_cache_due_rows,
        file_cache_failed_rows,
        recoverable_jobs,
        cancelling_jobs,
        job_concurrency: runtime_config.job_concurrency,
        active_transfer_jobs: app_context.transfer_runtime.active_transfer_jobs_count(),
        progress_edit_interval_seconds: runtime_config.progress_edit_interval_seconds,
        file_delete_delay_minutes: runtime_config.file_delete_delay_minutes,
        file_gc_interval_seconds: runtime_config.file_gc_interval_seconds,
    })
}

/// 读取 file_cache 按状态聚合的只读汇总。
pub(in crate::tgbot::transfer) async fn list_file_cache_status_summaries()
-> anyhow::Result<Vec<FileCacheStatusSummary>> {
    let db_conn = db::get_db().await?;
    let rows = db::file_cache::Entity::find()
        .select_only()
        .column(db::file_cache::Column::Status)
        .column(db::file_cache::Column::ActiveRefs)
        .into_tuple::<(String, i32)>()
        .all(db_conn)
        .await?;
    let mut summary: BTreeMap<String, (i64, i64)> = BTreeMap::new();
    for (status, active_refs) in rows {
        let entry = summary.entry(status).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += active_refs as i64;
    }
    Ok(summary
        .into_iter()
        .map(|(status, (count, active_refs))| FileCacheStatusSummary {
            status,
            count,
            active_refs,
        })
        .collect())
}

/// 分页读取最近更新的 file_cache 记录。
pub(in crate::tgbot::transfer) async fn list_recent_file_cache_snapshots(
    limit: u64,
    page: u64,
) -> anyhow::Result<Vec<FileCacheSnapshot>> {
    let db_conn = db::get_db().await?;
    let limit = limit.clamp(1, 50);
    let offset = page.saturating_sub(1).saturating_mul(limit);
    let rows = db::file_cache::Entity::find()
        .select_only()
        .column(db::file_cache::Column::OwnerClientRole)
        .column(db::file_cache::Column::FileKey)
        .column(db::file_cache::Column::Status)
        .column(db::file_cache::Column::ActiveRefs)
        .column(db::file_cache::Column::SizeBytes)
        .column(db::file_cache::Column::TdFileId)
        .column(db::file_cache::Column::LocalPath)
        .column(db::file_cache::Column::DeleteAfter)
        .column(db::file_cache::Column::LastUsedAt)
        .column(db::file_cache::Column::UpdatedAt)
        .column(db::file_cache::Column::LastError)
        .order_by_desc(db::file_cache::Column::UpdatedAt)
        .limit(limit)
        .offset(offset)
        .into_tuple::<(
            String,
            String,
            String,
            i32,
            Option<i64>,
            Option<i32>,
            Option<String>,
            Option<chrono::DateTime<chrono::FixedOffset>>,
            chrono::DateTime<chrono::FixedOffset>,
            chrono::DateTime<chrono::FixedOffset>,
            Option<String>,
        )>()
        .all(db_conn)
        .await?;

    Ok(rows
        .into_iter()
        .map(
            |(
                owner_client_role,
                file_key,
                status,
                active_refs,
                size_bytes,
                td_file_id,
                local_path,
                delete_after,
                last_used_at,
                updated_at,
                last_error,
            )| FileCacheSnapshot {
                owner_client_role,
                file_key,
                status,
                active_refs,
                size_bytes,
                td_file_id,
                local_path,
                delete_after,
                last_used_at,
                updated_at,
                last_error,
            },
        )
        .collect())
}

/// 统计状态命中数量。
fn count_statuses(statuses: &[String], expected: &[&str]) -> i64 {
    statuses
        .iter()
        .filter(|status| expected.contains(&status.as_str()))
        .count() as i64
}
