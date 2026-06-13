// 转存数据访问模块（数据库读写）：
// - 任务主表与子项状态管理
// - file_cache 引用计数与删除队列管理
// - 启动恢复任务扫描

mod account;
mod file_cache;
mod item;
mod job;
mod observability;
mod progress;
mod result;

#[cfg(test)]
mod tests;

pub(in crate::tgbot::transfer) use account::{
    PointLedgerEntry, PointLedgerPage, PointsChange, UserAccountSnapshot, change_points,
    ensure_user_account, get_user_account, list_point_ledger_page,
};
#[cfg(test)]
pub(super) use file_cache::{acquire_file_ref, release_job_file_refs};
pub(super) use file_cache::{
    claim_file_cache_for_delete, delete_file_cache, list_due_file_cache,
    mark_file_cache_delete_failed, mark_file_cache_downloading, mark_file_cache_failed,
    mark_file_cache_ready,
};
pub(super) use item::{
    ensure_items_for_bundle, list_items_by_job, reconcile_items_for_bundle, set_item_status,
};
#[cfg(test)]
pub(super) use job::finish_uploaded_job;
pub(super) use job::{
    CreateJobBilling, cancel_job_now, create_job, find_job_by_request, finish_job,
    finish_job_with_item_statuses, finish_uploaded_job_with_item_statuses, get_job_status,
    list_cancelling_jobs, list_recoverable_jobs, mark_job_running, pause_job_with_owner_scope,
    request_cancel_job_with_owner_scope, update_result_message_link, wake_job_with_owner_scope,
};
pub(super) use observability::{
    list_file_cache_status_summaries, list_recent_file_cache_snapshots,
    list_transfer_health_snapshot,
};
pub(super) use progress::{
    find_active_job_by_source_target, find_active_job_id_by_source_target,
    find_success_job_by_source_target, get_job_progress_snapshot_for_actor,
    get_job_progress_snapshot_with_context, list_recent_job_snapshots_for_actor,
};
pub(super) use result::{
    ResultMessageRecord, list_result_messages_by_job, replace_result_messages_on_conn,
    update_result_message_record_link,
};

/// 主任务状态：等待后台执行。
pub(super) const JOB_STATUS_PENDING: &str = "pending";
/// 主任务状态：后台正在执行。
pub(super) const JOB_STATUS_RUNNING: &str = "running";
/// 主任务状态：用户手动暂停。
pub(super) const JOB_STATUS_PAUSED: &str = "paused";
/// 主任务状态：用户已请求停止，等待后台任务在安全点收尾。
pub(super) const JOB_STATUS_CANCELLING: &str = "cancelling";
/// 主任务状态：内部取消收尾中，已有执行者正在释放文件引用。
pub(super) const JOB_STATUS_CANCEL_FINALIZING: &str = "cancel_finalizing";
/// 主任务状态：用户停止完成。
pub(super) const JOB_STATUS_CANCELLED: &str = "cancelled";
/// 主任务状态：全部成功。
pub(super) const JOB_STATUS_SUCCESS: &str = "success";
/// 主任务状态：全部失败。
pub(super) const JOB_STATUS_FAILED: &str = "failed";
/// 主任务状态：部分成功、部分失败。
pub(super) const JOB_STATUS_PARTIAL: &str = "partial";

/// 子项状态：等待处理。
pub(super) const ITEM_STATUS_PENDING: &str = "pending";
/// 子项状态：正在准备上传内容，媒体通常处于下载或构造 InputMessageContent 阶段。
pub(super) const ITEM_STATUS_PREPARING: &str = "preparing";
/// 子项状态：已准备完成，等待整批上传。
pub(super) const ITEM_STATUS_PREPARED: &str = "prepared";
/// 子项状态：正在上传。
pub(super) const ITEM_STATUS_UPLOADING: &str = "uploading";
/// 子项状态：已成功上传。
pub(super) const ITEM_STATUS_SUCCESS: &str = "success";
/// 子项状态：准备或上传失败。
pub(super) const ITEM_STATUS_FAILED: &str = "failed";
/// 子项状态：已被用户停止。
pub(super) const ITEM_STATUS_CANCELLED: &str = "cancelled";
/// 子项状态：恢复对齐后源相册中已不存在，不再参与后续下载/上传。
pub(super) const ITEM_STATUS_OBSOLETE: &str = "obsolete";

/// file_cache 状态：等待首次下载或重新引用。
const FILE_CACHE_STATUS_PENDING: &str = "pending";
/// file_cache 状态：TDLib 正在下载。
pub(super) const FILE_CACHE_STATUS_DOWNLOADING: &str = "downloading";
/// file_cache 状态：本地文件已可用于上传。
const FILE_CACHE_STATUS_READY: &str = "ready";
/// file_cache 状态：下载或准备失败。
const FILE_CACHE_STATUS_FAILED: &str = "failed";
/// file_cache 状态：GC 已认领，正在删除本地文件。
const FILE_CACHE_STATUS_DELETING: &str = "deleting";
/// file_cache 状态：删除失败，后续 GC 会重试或新任务重新引用。
const FILE_CACHE_STATUS_DELETE_FAILED: &str = "delete_failed";
/// 引用文件时遇到 GC 正在删除，最多等待的轮数。
const FILE_CACHE_DELETING_RETRY_LIMIT: usize = 20;
/// 引用文件时遇到 GC 正在删除，每轮等待毫秒数。
const FILE_CACHE_DELETING_RETRY_DELAY_MS: u64 = 50;

/// 已成功转存任务的最小查询结果。
///
/// lookup 和启动查重只需要任务 ID、目标 chat、结果消息 ID 与结果链接，不需要读取 transfer_job 全字段。
#[derive(Debug, Clone)]
pub(super) struct SuccessfulJobResult {
    /// 已成功任务 ID，用于日志定位。
    pub id: i64,
    /// 目标转存 chat_id，用于历史链接失效时重新生成入口链接。
    pub target_chat_id: i64,
    /// 上传结果入口消息 ID；旧数据可能为空，无法刷新时继续使用已存链接。
    pub result_message_id: Option<i64>,
    /// 上传结果入口链接。
    pub result_message_link: String,
}

/// 进度快照内的主任务展示字段。
///
/// `/downloads` 与单任务进度面板只展示这些字段，单独建轻量结构可以避免查询整行 transfer_job。
#[derive(Debug, Clone)]
pub(super) struct JobProgressJob {
    /// 主键任务 ID。
    pub id: i64,
    /// 当前任务状态。
    pub status: String,
    /// 任务总条目数。
    pub total_items: i32,
    /// 目标转存 chat_id。
    pub target_chat_id: i64,
    /// 最后一次失败原因；仅失败/部分失败/取消异常时展示给 `/job status`。
    pub last_error: Option<String>,
    /// 创建时间，用于列表排序。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// 最后更新时间，用于页面展示。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

/// 单个转存任务的进度快照。
/// 用于 `/downloads` 命令汇总展示。
#[derive(Debug, Clone)]
pub(super) struct JobProgressSnapshot {
    /// 主任务展示字段。
    pub job: JobProgressJob,
    /// 尚未开始处理的子项数。
    pub pending_count: i32,
    /// 正在准备（通常是下载/构建上传内容）的子项数。
    pub preparing_count: i32,
    /// 已准备完成、等待整批上传的子项数。
    pub prepared_count: i32,
    /// 正在上传的子项数。
    pub uploading_count: i32,
    /// 已成功完成的子项数。
    pub success_count: i32,
    /// 已失败的子项数。
    pub failed_count: i32,
    /// 已取消的子项数。
    pub cancelled_count: i32,
    /// 当前存在实时下载进度的文件数。
    pub active_download_files: i32,
    /// 当前活跃下载已下载总字节数。
    pub active_downloaded_bytes: i64,
    /// 当前活跃下载总字节数。
    pub active_download_total_bytes: i64,
    /// 是否存在总大小未知的活跃下载。
    pub has_unknown_download_total: bool,
}

/// 主任务完成摘要。
///
/// finish 相关函数统一使用该结构传递终态字段，避免参数列表继续膨胀。
pub(super) struct FinishJobSummary {
    /// 最终成功子项数。
    pub ok_count: i32,
    /// 最终失败子项数。
    pub fail_count: i32,
    /// 最后一次错误信息。
    pub last_error: Option<String>,
    /// 上传结果入口消息 ID。
    pub result_message_id: Option<i64>,
    /// 上传结果入口链接。
    pub result_message_link: Option<String>,
    /// 上传结果入口列表；超过 10 个媒体时会包含多个 album 入口。
    pub result_messages: Vec<ResultMessageRecord>,
    /// 文件引用释放后的延迟删除分钟数。
    pub delay_minutes: i64,
}

/// 判断任务是否已经处于终态。
pub(super) fn is_finished_job_status(status: &str) -> bool {
    matches!(
        status,
        JOB_STATUS_SUCCESS | JOB_STATUS_FAILED | JOB_STATUS_PARTIAL | JOB_STATUS_CANCELLED
    )
}

/// 判断是否是文本占位 file_key。
fn is_text_file_key(file_key: &str) -> bool {
    file_key.starts_with("text:")
}

/// 统一生成 UTC+8 时间戳。
pub(super) fn now_utc8() -> chrono::DateTime<chrono::FixedOffset> {
    let Some(offset) = chrono::FixedOffset::east_opt(8 * 3600) else {
        tracing::error!("failed to build UTC+8 fixed offset, fallback to UTC");
        return chrono::Utc::now().fixed_offset();
    };
    chrono::Utc::now().with_timezone(&offset)
}

/// 转存系统健康快照。
///
/// 这个结构只读，不参与任务状态变更，供 `/health` 与排障日志展示。
#[derive(Debug, Clone)]
pub(super) struct TransferHealthSnapshot {
    /// 数据库里的任务总数。
    pub total_jobs: i64,
    /// 处于 pending/running/paused/cancelling 的活跃任务数。
    pub active_jobs: i64,
    /// 成功任务数。
    pub success_jobs: i64,
    /// 失败任务数。
    pub failed_jobs: i64,
    /// 停止完成任务数。
    pub cancelled_jobs: i64,
    /// 总子项数。
    pub total_items: i64,
    /// 正在准备的子项数。
    pub preparing_items: i64,
    /// 正在上传的子项数。
    pub uploading_items: i64,
    /// file_cache 记录数。
    pub file_cache_rows: i64,
    /// 引用数大于 0 的 file_cache 数量。
    pub file_cache_active_rows: i64,
    /// 待删除的 file_cache 数量。
    pub file_cache_due_rows: i64,
    /// 删除失败的 file_cache 数量。
    pub file_cache_failed_rows: i64,
    /// 待恢复任务数。
    pub recoverable_jobs: i64,
    /// 待收敛的 cancelling 任务数。
    pub cancelling_jobs: i64,
    /// 运行时并发上限。
    pub job_concurrency: usize,
    /// 真实执行中的并发任务数。
    pub active_transfer_jobs: usize,
    /// 进度消息编辑间隔。
    pub progress_edit_interval_seconds: u64,
    /// 文件删除延迟分钟数。
    pub file_delete_delay_minutes: i64,
    /// 文件 GC 间隔秒数。
    pub file_gc_interval_seconds: u64,
}

/// file_cache 只读汇总。
///
/// `/cache` 只需要这类汇总数据，不需要暴露完整 file_cache 行。
#[derive(Debug, Clone)]
pub(super) struct FileCacheStatusSummary {
    /// 缓存状态名。
    pub status: String,
    /// 该状态下的记录数。
    pub count: i64,
    /// 该状态下的引用数总和。
    pub active_refs: i64,
}

/// 单条 file_cache 观测快照。
///
/// 这个结构只用于 `/cache` 的分页展示和排障，不改变任何删除/引用逻辑。
#[derive(Debug, Clone)]
pub(super) struct FileCacheSnapshot {
    /// 所属 client 角色。
    pub owner_client_role: String,
    /// file_key。
    pub file_key: String,
    /// 当前状态。
    pub status: String,
    /// 引用数。
    pub active_refs: i32,
    /// 文件大小。
    pub size_bytes: Option<i64>,
    /// TDLib file_id。
    pub td_file_id: Option<i32>,
    /// 本地路径。
    pub local_path: Option<String>,
    /// 删除时间。
    pub delete_after: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// 最近使用时间。
    pub last_used_at: chrono::DateTime<chrono::FixedOffset>,
    /// 最后更新时间。
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    /// 最近错误。
    pub last_error: Option<String>,
}
