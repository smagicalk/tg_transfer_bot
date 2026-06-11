// `/job` 详情卡片按钮。
// 只负责根据任务状态构造 inline keyboard，不执行任务状态变更。

use crate::tgbot::send;
use crate::tgbot::transfer::store::JobProgressSnapshot;

use super::super::common::{CommandStyle, downloads_command, job_command as build_job_command};
use super::super::downloads::build_downloads_return_list_callback_data;
use super::super::menu::build_menu_home_callback_data;
use super::args::{JobCallbackAction, build_job_callback_data};
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
                &build_job_callback_data(JobCallbackAction::Stop, job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ));
        }
        rows.push(action_row);
    }

    rows.push(vec![
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
        send::build_copy_button(
            "复制列表命令",
            &downloads_command(Some(meta.list_filter), None, None, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_callback_button(
            "菜单",
            &build_menu_home_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]);
    rows.push(vec![send::build_copy_button(
        "复制详情命令",
        &build_job_command("st", job_id, CommandStyle::Short),
        tdlib_rs::enums::ButtonStyle::Default,
    )]);
    rows
}

#[cfg(test)]
mod tests {
    use super::{build_job_status_buttons, job_status_meta};
    use crate::tgbot::transfer::store;

    // 运行中任务详情应提供暂停/停止 callback 按钮，便于直接控制。
    #[test]
    fn test_build_job_status_buttons_for_running() {
        let buttons = build_job_status_buttons(&snapshot_with_status(store::JOB_STATUS_RUNNING));

        assert_eq!(buttons[0][0].text, "暂停");
        assert_eq!(buttons[0][1].text, "停止");
        assert_eq!(buttons[1][0].text, "刷新详情");
        assert_eq!(buttons[1][3].text, "菜单");
        assert!(matches!(
            buttons[0][0].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
        assert!(matches!(
            buttons[1][3].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
    }

    // paused 任务详情应提供恢复 callback 按钮。
    #[test]
    fn test_build_job_status_buttons_for_paused() {
        let buttons = build_job_status_buttons(&snapshot_with_status(store::JOB_STATUS_PAUSED));

        assert_eq!(buttons[0][0].text, "恢复");
        assert_eq!(buttons[0][1].text, "停止");
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

    fn snapshot_with_status(status: &str) -> store::JobProgressSnapshot {
        let now = store::now_utc8();
        store::JobProgressSnapshot {
            job: store::JobProgressJob {
                id: 42,
                target_chat_id: -100,
                status: status.to_owned(),
                total_items: 3,
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
