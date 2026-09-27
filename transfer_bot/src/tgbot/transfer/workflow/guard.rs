use crate::app_context::{SourceTargetCreateGuard, TransferJobGuard};

/// 检查指定任务是否正在当前进程内并发运行。
///
/// # 参数
/// - `app_context`: 全局应用上下文。
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - `true` 表示已有协程持有该任务排他锁；`false` 表示无进程内活跃协程。
pub(in crate::tgbot::transfer) async fn is_job_running_in_process(
    app_context: &crate::app_context::AppContext,
    job_id: i64,
) -> bool {
    app_context
        .transfer_guards
        .is_job_running_in_process(job_id)
        .await
}

/// 尝试获取指定任务的进程内排他执行锁（TransferJobGuard）。
///
/// 防止同一任务被多次并发派发或同时被恢复协程与新任务协程执行。
///
/// # 参数
/// - `app_context`: 全局应用上下文。
/// - `job_id`: 任务 ID。
///
/// # 返回值
/// - `Some(TransferJobGuard)` 成功锁定；`None` 表示已有其它协程持有。
pub(super) async fn acquire_job_guard(
    app_context: &crate::app_context::AppContext,
    job_id: i64,
) -> Option<TransferJobGuard> {
    app_context.transfer_guards.acquire_job_guard(job_id).await
}

/// 获取按 `(source_link, target_chat_id)` 维度的创建排他锁。
///
/// 避免在高并发下，两个用户或多次快速点击同时通过查重检测并重复创建同一目标的转存任务。
///
/// # 参数
/// - `app_context`: 全局应用上下文。
/// - `source_link`: 来源 Telegram 链接。
/// - `target_chat_id`: 目标频道 ID。
///
/// # 返回值
/// - 作用域守卫 `SourceTargetCreateGuard`，drop 时自动释放。
pub(super) async fn acquire_source_target_create_guard(
    app_context: &crate::app_context::AppContext,
    source_link: String,
    target_chat_id: i64,
) -> SourceTargetCreateGuard {
    app_context
        .transfer_guards
        .acquire_source_target_create_guard(source_link, target_chat_id)
        .await
}
