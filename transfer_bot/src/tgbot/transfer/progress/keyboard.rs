// 转存进度面板的按钮构造。
// 这里只生成 Telegram inline keyboard，具体发送/编辑由上层 progress 模块负责。

use crate::tgbot::transfer::command::{
    build_downloads_filter_button_data, build_downloads_status_button_data,
    build_job_list_button_meta, build_job_pause_button_data, build_job_resume_button_data,
    build_job_status_button_data, build_job_stop_button_data, build_menu_home_button_data,
};
use crate::tgbot::transfer::store;

/// 构造进度面板按钮。
pub(super) fn build_transfer_progress_keyboard(
    job_id: Option<i64>,
    job_status: Option<&str>,
    _source_link: &str,
    _target_chat_id: i64,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let (list_status, list_label) = job_status
        .map(build_job_list_button_meta)
        .unwrap_or(("run", "查看运行列表"));
    let mut rows = vec![vec![
        crate::tgbot::send::build_callback_button(
            list_label,
            &build_downloads_status_button_data(list_status, 8),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_callback_button(
            "菜单",
            &build_menu_home_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]];

    if let Some(job_id) = job_id {
        rows.extend(build_job_control_rows(job_id, job_status));
    }

    crate::tgbot::send::build_inline_keyboard(rows)
}

/// 按任务状态构造可点击控制按钮和必要的复制按钮。
///
/// 进度面板可能被最终结果复用，因此这里不能对 cancelled/cancelling 再展示暂停按钮。
/// 正文已经保留完整命令，按钮区只保留真正的交互控制和必要的 `job_id` 复制。
fn build_job_control_rows(
    job_id: i64,
    job_status: Option<&str>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut action_row = vec![crate::tgbot::send::build_callback_button(
        "查看任务详情",
        &build_job_status_button_data(job_id),
        tdlib_rs::enums::ButtonStyle::Primary,
    )];

    match job_status {
        Some(store::JOB_STATUS_PAUSED) => {
            action_row.push(crate::tgbot::send::build_callback_button(
                "恢复",
                &build_job_resume_button_data(job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ));
            action_row.push(crate::tgbot::send::build_callback_button(
                "停止",
                &build_job_stop_button_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ));
            vec![action_row]
        }
        Some(store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING)
        | Some(store::JOB_STATUS_CANCELLED) => {
            vec![
                action_row,
                vec![crate::tgbot::send::build_copy_button(
                    "复制 job_id",
                    &job_id.to_string(),
                    tdlib_rs::enums::ButtonStyle::Default,
                )],
            ]
        }
        _ => {
            action_row.push(crate::tgbot::send::build_callback_button(
                "暂停",
                &build_job_pause_button_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ));
            action_row.push(crate::tgbot::send::build_callback_button(
                "停止",
                &build_job_stop_button_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ));
            vec![action_row]
        }
    }
}

/// 构造最终结果按钮。
pub(super) fn build_transfer_result_keyboard(
    _source_link: &str,
    _target_chat_id: i64,
    job_id: Option<i64>,
    result_link: Option<&str>,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let mut rows = Vec::new();
    if let Some(result_link) = result_link {
        // 只有 TDLib 返回或本模块兜底生成的 HTTP(S) 链接才放“打开转存消息”按钮；客户端 deeplink 不稳定，
        // 放到 URL 按钮里会造成点击无反应。
        let mut result_row = Vec::new();
        if crate::tgbot::send::is_openable_url(result_link) {
            result_row.push(crate::tgbot::send::build_url_button(
                "打开转存消息",
                result_link,
                tdlib_rs::enums::ButtonStyle::Primary,
            ));
        }
        result_row.push(crate::tgbot::send::build_copy_button(
            if crate::tgbot::send::is_openable_url(result_link) {
                "复制结果链接"
            } else {
                "复制结果定位"
            },
            result_link,
            if crate::tgbot::send::is_openable_url(result_link) {
                tdlib_rs::enums::ButtonStyle::Default
            } else {
                tdlib_rs::enums::ButtonStyle::Primary
            },
        ));
        rows.push(result_row);
    }

    if let Some(job_id) = job_id {
        rows.push(vec![crate::tgbot::send::build_callback_button(
            "查看任务详情",
            &build_job_status_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Primary,
        )]);
    }

    let list_filter = if result_link.is_some() {
        "done"
    } else {
        "fail"
    };
    let list_label = if result_link.is_some() {
        "查看完成列表"
    } else {
        "查看失败列表"
    };
    let mut navigation_row = Vec::new();
    if let Some(callback_data) = build_downloads_filter_button_data(list_filter, 8) {
        navigation_row.push(crate::tgbot::send::build_callback_button(
            list_label,
            &callback_data,
            tdlib_rs::enums::ButtonStyle::Primary,
        ));
    }
    navigation_row.push(crate::tgbot::send::build_callback_button(
        "菜单",
        &build_menu_home_button_data(),
        tdlib_rs::enums::ButtonStyle::Default,
    ));
    rows.push(navigation_row);

    crate::tgbot::send::build_inline_keyboard(rows)
}
