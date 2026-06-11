// transfer_job 主表访问入口。
// 具体实现按职责拆到子模块，避免任务查询、控制和终态写入混在一个文件里。

mod cancel;
mod control;
mod create;
mod finish;
mod query;

pub(in crate::tgbot::transfer) use cancel::cancel_job_now;
pub(in crate::tgbot::transfer) use control::{
    mark_job_running, pause_job_with_owner_scope, request_cancel_job_with_owner_scope,
    wake_job_with_owner_scope,
};
pub(in crate::tgbot::transfer) use create::{CreateJobBilling, create_job};
#[cfg(test)]
pub(in crate::tgbot::transfer) use finish::finish_uploaded_job;
pub(in crate::tgbot::transfer) use finish::{
    finish_job, finish_job_with_item_statuses, finish_uploaded_job_with_item_statuses,
    update_result_message_link,
};
pub(in crate::tgbot::transfer) use query::{
    find_job_by_request, get_job_status, list_cancelling_jobs, list_recoverable_jobs,
};
