// 任务 hub 最近任务快捷按钮。
// 这里只生成 `/job` callback，不直接改任务状态。

use crate::tgbot::send;

use super::super::super::super::store;
use super::super::super::job::{
    build_job_pause_callback_data, build_job_resume_callback_data, build_job_status_callback_data,
    build_job_stop_callback_data,
};
/// 最近任务快捷按钮。
///
/// 这些按钮只携带 job_id，并复用 `/job` 详情 callback；菜单不复制任务详情逻辑。
pub(super) fn recent_job_buttons(
    recent_jobs: &[store::JobProgressSnapshot],
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    if recent_jobs.is_empty() {
        return Vec::new();
    }

    recent_jobs
        .iter()
        .take(5)
        .map(|snapshot| {
            let status = snapshot.job.status.as_str();
            let style = if matches!(
                status,
                store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING | store::JOB_STATUS_PAUSED
            ) {
                tdlib_rs::enums::ButtonStyle::Primary
            } else {
                tdlib_rs::enums::ButtonStyle::Default
            };
            let job_id = snapshot.job.id;
            let mut row = vec![send::build_callback_button(
                &format!("#{} {}", snapshot.job.id, snapshot.job.status),
                &build_job_status_callback_data(snapshot.job.id),
                style,
            )];
            row.extend(recent_job_control_buttons(job_id, status));
            row
        })
        .collect::<Vec<_>>()
}

/// 最近任务快捷控制。
fn recent_job_control_buttons(
    job_id: i64,
    status: &str,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    if matches!(
        status,
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
    ) {
        return vec![
            send::build_callback_button(
                "暂停",
                &build_job_pause_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "停止",
                &build_job_stop_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ];
    }

    if status == store::JOB_STATUS_PAUSED {
        return vec![
            send::build_callback_button(
                "恢复",
                &build_job_resume_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "停止",
                &build_job_stop_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ];
    }

    Vec::new()
}
