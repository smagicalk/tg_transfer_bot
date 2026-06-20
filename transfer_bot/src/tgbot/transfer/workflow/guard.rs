use crate::app_context::{SourceTargetCreateGuard, TransferJobGuard};

pub(in crate::tgbot::transfer) async fn is_job_running_in_process(
    app_context: &crate::app_context::AppContext,
    job_id: i64,
) -> bool {
    app_context
        .transfer_guards
        .is_job_running_in_process(job_id)
        .await
}

pub(super) async fn acquire_job_guard(
    app_context: &crate::app_context::AppContext,
    job_id: i64,
) -> Option<TransferJobGuard> {
    app_context.transfer_guards.acquire_job_guard(job_id).await
}

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
