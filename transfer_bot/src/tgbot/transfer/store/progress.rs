// 任务进度查询与历史任务检索。

use std::collections::{HashMap, HashSet};

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
    JOB_STATUS_RUNNING, JOB_STATUS_SUCCESS, JobProgressJob, JobProgressSnapshot,
    SuccessfulJobResult, is_text_file_key,
};

/// 按 `source_link + target_chat_id` 查找最近已成功转存且已保存入口链接的任务。
/// 命中后优先复用历史链接；如果旧链接不可点击，调用方会用 result_message_id 刷新。
pub(in crate::tgbot::transfer) async fn find_success_job_by_source_target(
    source_link: &str,
    target_chat_id: i64,
) -> anyhow::Result<Option<SuccessfulJobResult>> {
    let db_conn = db::get_db().await?;
    let row = db::transfer_job::Entity::find()
        .select_only()
        .column(db::transfer_job::Column::Id)
        .column(db::transfer_job::Column::TargetChatId)
        .column(db::transfer_job::Column::ResultMessageId)
        .column(db::transfer_job::Column::ResultMessageLink)
        .filter(db::transfer_job::Column::SourceLink.eq(source_link.to_owned()))
        .filter(db::transfer_job::Column::TargetChatId.eq(target_chat_id))
        .filter(db::transfer_job::Column::Status.eq(JOB_STATUS_SUCCESS))
        .filter(db::transfer_job::Column::ResultMessageLink.is_not_null())
        .order_by_desc(db::transfer_job::Column::FinishedAt)
        .into_tuple::<(i64, i64, Option<i64>, Option<String>)>()
        .one(db_conn)
        .await?;

    // 数据库层已经过滤非空链接，但实体字段类型仍是 Option，这里再做一次防御转换。
    Ok(row.and_then(
        |(id, target_chat_id, result_message_id, result_message_link)| {
            result_message_link.map(|result_message_link| SuccessfulJobResult {
                id,
                target_chat_id,
                result_message_id,
                result_message_link,
            })
        },
    ))
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

/// 按 `source_link + target_chat_id` 查找最近进行中任务的 ID。
///
/// 进度面板每几秒轮询一次，只需要 job_id 再读取轻量进度快照，避免频繁读取 transfer_job 全字段。
pub(in crate::tgbot::transfer) async fn find_active_job_id_by_source_target(
    source_link: &str,
    target_chat_id: i64,
) -> anyhow::Result<Option<i64>> {
    let db_conn = db::get_db().await?;
    db::transfer_job::Entity::find()
        .select_only()
        .column(db::transfer_job::Column::Id)
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
        .into_tuple::<i64>()
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
        .select_only()
        .column(db::transfer_job::Column::Id)
        .column(db::transfer_job::Column::Status)
        .column(db::transfer_job::Column::TotalItems)
        .column(db::transfer_job::Column::TargetChatId)
        .column(db::transfer_job::Column::CreatedAt)
        .column(db::transfer_job::Column::UpdatedAt)
        .filter(db::transfer_job::Column::RequestChatId.eq(request_chat_id))
        .order_by_desc(db::transfer_job::Column::CreatedAt)
        .limit(limit)
        .into_tuple::<(
            i64,
            String,
            i32,
            i64,
            chrono::DateTime<chrono::FixedOffset>,
            chrono::DateTime<chrono::FixedOffset>,
        )>()
        .all(db_conn)
        .await?
        .into_iter()
        .map(
            |(id, status, total_items, target_chat_id, created_at, updated_at)| JobProgressJob {
                id,
                status,
                total_items,
                target_chat_id,
                created_at,
                updated_at,
            },
        )
        .collect::<Vec<_>>();

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
    get_job_progress_snapshot_with_request_chat(job_id, None).await
}

/// 查询当前请求聊天可见的单个任务进度快照。
///
/// `/job status` 是用户手动输入 job_id 的命令，必须同时校验 request_chat_id，
/// 避免一个聊天通过猜测 job_id 看到其他聊天发起的任务详情。
pub(in crate::tgbot::transfer) async fn get_job_progress_snapshot_for_request_chat(
    job_id: i64,
    request_chat_id: i64,
) -> anyhow::Result<Option<JobProgressSnapshot>> {
    get_job_progress_snapshot_with_request_chat(job_id, Some(request_chat_id)).await
}

/// 查询单个任务进度快照的内部实现。
///
/// `request_chat_id` 为空时用于进度面板内部轮询；非空时用于命令权限边界。
async fn get_job_progress_snapshot_with_request_chat(
    job_id: i64,
    request_chat_id: Option<i64>,
) -> anyhow::Result<Option<JobProgressSnapshot>> {
    let db_conn = db::get_db().await?;
    let mut query = db::transfer_job::Entity::find()
        .select_only()
        .column(db::transfer_job::Column::Id)
        .column(db::transfer_job::Column::Status)
        .column(db::transfer_job::Column::TotalItems)
        .column(db::transfer_job::Column::TargetChatId)
        .column(db::transfer_job::Column::CreatedAt)
        .column(db::transfer_job::Column::UpdatedAt)
        .filter(db::transfer_job::Column::Id.eq(job_id));

    if let Some(request_chat_id) = request_chat_id {
        query = query.filter(db::transfer_job::Column::RequestChatId.eq(request_chat_id));
    }

    let Some((id, status, total_items, target_chat_id, created_at, updated_at)) = query
        .into_tuple::<(
            i64,
            String,
            i32,
            i64,
            chrono::DateTime<chrono::FixedOffset>,
            chrono::DateTime<chrono::FixedOffset>,
        )>()
        .one(db_conn)
        .await?
    else {
        return Ok(None);
    };
    let job = JobProgressJob {
        id,
        status,
        total_items,
        target_chat_id,
        created_at,
        updated_at,
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
    jobs: Vec<JobProgressJob>,
) -> anyhow::Result<Vec<JobProgressSnapshot>> {
    let db_conn = db::get_db().await?;
    let job_ids = jobs.iter().map(|job| job.id).collect::<Vec<_>>();
    let items = db::transfer_item::Entity::find()
        .select_only()
        .column(db::transfer_item::Column::JobId)
        .column(db::transfer_item::Column::FileKey)
        .column(db::transfer_item::Column::Status)
        .filter(db::transfer_item::Column::JobId.is_in(job_ids))
        .into_tuple::<(i64, String, String)>()
        .all(db_conn)
        .await?;
    let file_keys = items
        .iter()
        .map(|(_, file_key, _)| file_key.clone())
        .filter(|file_key| !is_text_file_key(file_key))
        .collect::<HashSet<_>>();
    let file_cache_rows = if file_keys.is_empty() {
        vec![]
    } else {
        db::file_cache::Entity::find()
            .select_only()
            .column(db::file_cache::Column::FileKey)
            .column(db::file_cache::Column::Status)
            .column(db::file_cache::Column::TdFileId)
            .column(db::file_cache::Column::SizeBytes)
            .filter(db::file_cache::Column::FileKey.is_in(file_keys))
            .into_tuple::<(String, String, Option<i32>, Option<i64>)>()
            .all(db_conn)
            .await?
    };
    let file_cache_map = file_cache_rows
        .into_iter()
        .map(|(file_key, status, td_file_id, size_bytes)| {
            (
                file_key,
                FileCacheProgressRow {
                    status,
                    td_file_id,
                    size_bytes,
                },
            )
        })
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
    for (job_id, file_key, status) in items {
        let Some(snapshot) = count_map.get_mut(&job_id) else {
            continue;
        };

        match status.as_str() {
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
        if status != ITEM_STATUS_PREPARING {
            continue;
        }

        let Some(file_cache) = file_cache_map.get(&file_key) else {
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
    snapshots.sort_by_key(|snapshot| std::cmp::Reverse(snapshot.job.created_at));
    Ok(snapshots)
}

/// 进度统计所需的 file_cache 字段。
///
/// 不包含 local_path/last_error/引用计数等无关字段，减少 `/downloads` 和进度面板读取成本。
struct FileCacheProgressRow {
    /// 缓存状态，用于判断文件是否正在下载。
    status: String,
    /// TDLib 文件 ID，用于读取当前进程内的实时下载进度。
    td_file_id: Option<i32>,
    /// 数据库中记录的文件大小，用作 TDLib 尚未推送进度时的兜底总量。
    size_bytes: Option<i64>,
}
