// `/job` 详情卡片按钮。
// 只负责根据任务状态构造 inline keyboard，不执行任务状态变更。

use crate::tgbot::send;
use crate::tgbot::transfer::store::JobProgressSnapshot;

use super::super::common::build_refresh_return_menu_row;
use super::super::downloads::build_downloads_return_list_callback_data;
use super::super::menu::build_menu_home_callback_data;
use super::args::{JobCallbackAction, build_job_callback_data};
use super::build_job_stop_execute_callback_data;
use super::status_meta::job_status_meta;

/// 构造单任务详情按钮。
pub(super) fn build_job_status_buttons(
    snapshot: &JobProgressSnapshot,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let job_id = snapshot.job.id;
    let status = snapshot.job.status.as_str();
    let meta = job_status_meta(status);
    let mut rows = Vec::new();

    if meta.show_pause || meta.show_resume || meta.show_stop {
        let mut action_row = Vec::new();
        if meta.show_pause {
            action_row.push(send::build_callback_button(
                "暂停",
                &build_job_callback_data(JobCallbackAction::Pause, job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ));
        }
        if meta.show_resume {
            action_row.push(send::build_callback_button(
                "恢复",
                &build_job_callback_data(JobCallbackAction::Resume, job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ));
        }
        if meta.show_stop {
            action_row.push(send::build_callback_button(
                "停止",
                &build_job_callback_data(JobCallbackAction::StopConfirm, job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ));
        }
        rows.push(action_row);
    }

    rows.push(build_refresh_return_menu_row(
        send::build_callback_button(
            "刷新详情",
            &build_job_callback_data(JobCallbackAction::Status, job_id),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_callback_button(
            "返回列表",
            &build_downloads_return_list_callback_data(status, 8),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_callback_button(
            "菜单",
            &build_menu_home_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ));
    rows
}

/// 构造停止确认页按钮。
///
/// “确认停止”单独占一行，避免和返回按钮挤在一起导致误触。
pub(super) fn build_job_stop_confirm_buttons(
    snapshot: &JobProgressSnapshot,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let job_id = snapshot.job.id;
    let status = snapshot.job.status.as_str();
    vec![
        vec![send::build_callback_button(
            "确认停止",
            &build_job_stop_execute_callback_data(job_id),
            tdlib_rs::enums::ButtonStyle::Default,
        )],
        build_refresh_return_menu_row(
            send::build_callback_button(
                "返回详情",
                &build_job_callback_data(JobCallbackAction::Status, job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "返回列表",
                &build_downloads_return_list_callback_data(status, 8),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "菜单",
                &build_menu_home_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::{build_job_status_buttons, build_job_stop_confirm_buttons, job_status_meta};
    use crate::tgbot::transfer::store;
    use base64::{Engine as _, engine::general_purpose};

    // 运行中任务详情应把停止按钮导向确认页，避免误触后直接停止。
    #[test]
    fn test_build_job_status_buttons_for_running() {
        let buttons = build_job_status_buttons(&snapshot_with_status(store::JOB_STATUS_RUNNING));

        assert_eq!(buttons[0][0].text, "暂停");
        assert_eq!(buttons[0][1].text, "停止");
        assert_eq!(decoded_callback_data(&buttons[0][1]), "j:sc:42");
        assert_eq!(buttons[1][0].text, "刷新详情");
        assert_eq!(buttons[1][2].text, "菜单");
        assert!(matches!(
            buttons[0][0].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
        assert!(matches!(
            buttons[1][2].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
    }

    // paused 任务详情应提供恢复 callback 按钮。
    #[test]
    fn test_build_job_status_buttons_for_paused() {
        let buttons = build_job_status_buttons(&snapshot_with_status(store::JOB_STATUS_PAUSED));

        assert_eq!(buttons[0][0].text, "恢复");
        assert_eq!(buttons[0][1].text, "停止");
        assert_eq!(decoded_callback_data(&buttons[0][1]), "j:sc:42");
    }

    // 任务详情里的返回列表按钮应直接回到对应的 downloads 筛选入口。
    #[test]
    fn test_build_job_status_buttons_has_return_list_button() {
        let buttons = build_job_status_buttons(&snapshot_with_status(store::JOB_STATUS_RUNNING));

        assert_eq!(buttons[1][1].text, "返回列表");
        assert!(matches!(
            buttons[1][1].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
        assert_eq!(buttons.len(), 2);
    }

    // 任务状态应映射到最接近的 downloads 筛选。
    #[test]
    fn test_job_status_list_filter() {
        assert_eq!(
            job_status_meta(store::JOB_STATUS_RUNNING).list_filter,
            "run"
        );
        assert_eq!(
            job_status_meta(store::JOB_STATUS_PAUSED).list_filter,
            "pause"
        );
        assert_eq!(
            job_status_meta(store::JOB_STATUS_SUCCESS).list_filter,
            "done"
        );
        assert_eq!(
            job_status_meta(store::JOB_STATUS_FAILED).list_filter,
            "fail"
        );
    }

    // 停止确认页里只有“确认停止”会执行真实 stop，返回按钮只刷新详情或列表。
    #[test]
    fn test_build_job_stop_confirm_buttons() {
        let buttons =
            build_job_stop_confirm_buttons(&snapshot_with_status(store::JOB_STATUS_RUNNING));

        assert_eq!(buttons[0][0].text, "确认停止");
        assert_eq!(decoded_callback_data(&buttons[0][0]), "j:s:42");
        assert_eq!(buttons[1][0].text, "返回详情");
        assert_eq!(decoded_callback_data(&buttons[1][0]), "j:st:42");
        assert_eq!(buttons[1][1].text, "返回列表");
        assert_eq!(buttons[1][2].text, "菜单");
        assert_eq!(buttons.len(), 2);
    }

    fn decoded_callback_data(button: &tdlib_rs::types::InlineKeyboardButton) -> String {
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &button.r#type else {
            panic!("button must be callback");
        };
        String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap()
    }

    fn snapshot_with_status(status: &str) -> store::JobProgressSnapshot {
        let now = store::now_utc8();
        store::JobProgressSnapshot {
            job: store::JobProgressJob {
                id: 42,
                target_chat_id: -100,
                status: status.to_owned(),
                total_items: 3,
                last_error: None,
                created_at: now,
                updated_at: now,
            },
            pending_count: 1,
            preparing_count: 0,
            prepared_count: 1,
            uploading_count: 0,
            success_count: 1,
            failed_count: 0,
            cancelled_count: 0,
            active_download_files: 0,
            active_downloaded_bytes: 0,
            active_download_total_bytes: 0,
            has_unknown_download_total: false,
        }
    }
}
