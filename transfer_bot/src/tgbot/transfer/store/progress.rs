// 任务进度查询与历史任务检索。

use std::collections::HashMap;

use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;

use crate::db;
use crate::tgbot::queue;

use super::{
    FILE_CACHE_STATUS_DOWNLOADING, ITEM_STATUS_CANCELLED, ITEM_STATUS_FAILED, ITEM_STATUS_PENDING,
    ITEM_STATUS_PREPARED, ITEM_STATUS_PREPARING, ITEM_STATUS_SUCCESS, ITEM_STATUS_UPLOADING,
    JOB_STATUS_CANCEL_FINALIZING, JOB_STATUS_CANCELLING, JOB_STATUS_PAUSED, JOB_STATUS_PENDING,
    JOB_STATUS_RUNNING, JOB_STATUS_SUCCESS, JobProgressSnapshot, is_text_file_key,
};

/// 按 `source_link + target_chat_id` 查找最近已成功转存且已保存入口链接的任务。
/// 命中后可以直接复用历史链接，避免重复转存。
pub(in crate::tgbot::transfer) async fn find_success_job_by_source_target(
    source_link: &str,
    target_chat_id: i64,
) -> anyhow::Result<Option<db::transfer_job::Model>> {
    let db_conn = db::get_db().await?;
    db::transfer_job::Entity::find()
        .filter(db::transfer_job::Column::SourceLink.eq(source_link.to_owned()))
        .filter(db::transfer_job::Column::TargetChatId.eq(target_chat_id))
        .filter(db::transfer_job::Column::Status.eq(JOB_STATUS_SUCCESS))
        .filter(db::transfer_job::Column::ResultMessageLink.is_not_null())
        .order_by_desc(db::transfer_job::Column::FinishedAt)
        .one(db_conn)
        .await
        .map_err(Into::into)
}

/// 按 `source_link + target_chat_id` 查找最近进行中的任务。
/// 用于阻止同一目标的重复转存并返回明确提示。
pub(in crate::tgbot::transfer) async fn find_active_job_by_source_target(
    source_link: &str,
    target_chat_id: i64,
) -> anyhow::Result<Option<db::transfer_job::Model>> {
    let db_conn = db::get_db().await?;
    db::transfer_job::Entity::find()
        .filter(db::transfer_job::Column::SourceLink.eq(source_link.to_owned()))
        .filter(db::transfer_job::Column::TargetChatId.eq(target_chat_id))
        .filter(
            Condition::any()
                .add(db::transfer_job::Column::Status.eq(JOB_STATUS_PENDING))
                .add(db::transfer_job::Column::Status.eq(JOB_STATUS_RUNNING))
                .add(db::transfer_job::Column::Status.eq(JOB_STATUS_PAUSED))
                .add(db::transfer_job::Column::Status.eq(JOB_STATUS_CANCELLING))
                .add(db::transfer_job::Column::Status.eq(JOB_STATUS_CANCEL_FINALIZING)),
        )
        .order_by_desc(db::transfer_job::Column::CreatedAt)
        .one(db_conn)
        .await
        .map_err(Into::into)
}

/// 查询某个请求聊天最近的任务列表，并汇总每个任务的子项状态。
/// 这是 `/downloads` 命令的基础数据源。
pub(in crate::tgbot::transfer) async fn list_recent_job_snapshots(
    request_chat_id: i64,
    limit: u64,
) -> anyhow::Result<Vec<JobProgressSnapshot>> {
    let db_conn = db::get_db().await?;
    let jobs = db::transfer_job::Entity::find()
        .filter(db::transfer_job::Column::RequestChatId.eq(request_chat_id))
        .order_by_desc(db::transfer_job::Column::CreatedAt)
        .limit(limit)
        .all(db_conn)
        .await?;

    if jobs.is_empty() {
        return Ok(vec![]);
    }

    build_job_progress_snapshots(jobs).await
}

/// 查询单个任务的进度快照。
///
/// `/transfer` 进度面板会按 job_id 轮询该快照，然后编辑同一条消息。
pub(in crate::tgbot::transfer) async fn get_job_progress_snapshot(
    job_id: i64,
) -> anyhow::Result<Option<JobProgressSnapshot>> {
    let db_conn = db::get_db().await?;
    let Some(job) = db::transfer_job::Entity::find_by_id(job_id)
        .one(db_conn)
        .await?
    else {
        return Ok(None);
    };
    Ok(build_job_progress_snapshots(vec![job])
        .await?
        .into_iter()
        .next())
}

/// 根据任务列表批量构造进度快照。
///
/// 该函数集中处理子项状态统计和 TDLib 实时下载进度，避免 `/downloads`
/// 与单任务进度面板各自实现一套统计逻辑。
async fn build_job_progress_snapshots(
    jobs: Vec<db::transfer_job::Model>,
) -> anyhow::Result<Vec<JobProgressSnapshot>> {
    let db_conn = db::get_db().await?;
    let job_ids = jobs.iter().map(|job| job.id).collect::<Vec<_>>();
    let items = db::transfer_item::Entity::find()
        .filter(db::transfer_item::Column::JobId.is_in(job_ids))
        .all(db_conn)
        .await?;
    let file_keys = items
        .iter()
        .map(|item| item.file_key.clone())
        .filter(|file_key| !is_text_file_key(file_key))
        .collect::<Vec<_>>();
    let file_cache_rows = if file_keys.is_empty() {
        vec![]
    } else {
        db::file_cache::Entity::find()
            .filter(db::file_cache::Column::FileKey.is_in(file_keys))
            .all(db_conn)
            .await?
    };
    let file_cache_map = file_cache_rows
        .into_iter()
        .map(|row| (row.file_key.clone(), row))
        .collect::<HashMap<_, _>>();

    let mut count_map: HashMap<i64, JobProgressSnapshot> = HashMap::new();
    for job in jobs {
        count_map.insert(
            job.id,
            JobProgressSnapshot {
                job,
                pending_count: 0,
                preparing_count: 0,
                prepared_count: 0,
                uploading_count: 0,
                success_count: 0,
                failed_count: 0,
                cancelled_count: 0,
                active_download_files: 0,
                active_downloaded_bytes: 0,
                active_download_total_bytes: 0,
                has_unknown_download_total: false,
            },
        );
    }

    // 逐条累加子项状态。
    for item in items {
        let Some(snapshot) = count_map.get_mut(&item.job_id) else {
            continue;
        };

        match item.status.as_str() {
            ITEM_STATUS_PENDING => snapshot.pending_count += 1,
            ITEM_STATUS_PREPARING => snapshot.preparing_count += 1,
            ITEM_STATUS_PREPARED => snapshot.prepared_count += 1,
            ITEM_STATUS_UPLOADING => snapshot.uploading_count += 1,
            ITEM_STATUS_SUCCESS => snapshot.success_count += 1,
            ITEM_STATUS_FAILED => snapshot.failed_count += 1,
            ITEM_STATUS_CANCELLED => snapshot.cancelled_count += 1,
            // 兼容历史未知状态：不参与展示计数。
            _ => {}
        }

        // `preparing + file_cache.downloading` 才视为真正下载中的条目。
        if item.status != ITEM_STATUS_PREPARING {
            continue;
        }

        let Some(file_cache) = file_cache_map.get(&item.file_key) else {
            continue;
        };
        if file_cache.status != FILE_CACHE_STATUS_DOWNLOADING {
            continue;
        }

        snapshot.active_download_files += 1;
        let runtime_progress = file_cache.td_file_id.and_then(queue::get_download_progress);
        if let Some(progress) = runtime_progress {
            snapshot.active_downloaded_bytes += progress.downloaded_size.max(0);
            if let Some(total_size) = progress.total_size {
                snapshot.active_download_total_bytes += total_size.max(0);
            } else {
                snapshot.has_unknown_download_total = true;
            }
        } else {
            // TDLib 还没推送第一条进度时，先回退到数据库里的预估大小。
            if let Some(total_size) = file_cache.size_bytes {
                snapshot.active_download_total_bytes += total_size.max(0);
            } else {
                snapshot.has_unknown_download_total = true;
            }
        }
    }

    // 结果按创建时间倒序输出，和用户查看列表的直觉一致。
    let mut snapshots = count_map.into_values().collect::<Vec<_>>();
    snapshots.sort_by(|left, right| right.job.created_at.cmp(&left.job.created_at));
    Ok(snapshots)
}
