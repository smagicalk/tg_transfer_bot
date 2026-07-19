// 转存进度面板的按钮构造。
// 这里只生成 Telegram inline keyboard，具体发送/编辑由上层 progress 模块负责。

use crate::tgbot::transfer::command::{
    build_downloads_status_button_data, build_job_list_button_meta, build_menu_home_button_data,
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
    let navigation_row = vec![
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
    ];

    let mut rows = Vec::new();
    if let Some(job_id) = job_id {
        rows.extend(build_job_control_rows(job_id, job_status));
    }
    rows.push(navigation_row);

    crate::tgbot::send::build_inline_keyboard(rows)
}

/// 按任务状态构造可点击控制按钮。
///
/// 进度面板可能被最终结果复用，因此这里不能对 cancelled/cancelling 再展示暂停按钮。
/// 正文已经保留完整命令，按钮区只保留真正的交互控制。
fn build_job_control_rows(
    job_id: i64,
    job_status: Option<&str>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let status = match job_status {
        Some(store::JOB_STATUS_PAUSED) => "paused",
        Some(store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING) => "cancelling",
        Some(store::JOB_STATUS_CANCELLED) => "cancelled",
        _ => "running",
    };
    vec![crate::tgbot::transfer::outcome::build_job_action_row(
        status, job_id,
    )]
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
        // 放到 URL 按钮里会造成点击无反应。定位字符串已经在正文展示，因此这里不再重复给复制按钮。
        if crate::tgbot::send::is_openable_url(result_link) {
            rows.push(vec![crate::tgbot::send::build_url_button(
                "打开转存消息",
                result_link,
                tdlib_rs::enums::ButtonStyle::Primary,
            )]);
        }
    }

    if let Some(job_id) = job_id {
        rows.extend(
            crate::tgbot::transfer::outcome::build_result_navigation_rows(
                Some(job_id),
                "查看完成列表",
                "done",
            ),
        );
    } else {
        rows.extend(
            crate::tgbot::transfer::outcome::build_result_navigation_rows(
                None,
                "查看失败列表",
                "fail",
            ),
        );
    }

    crate::tgbot::send::build_inline_keyboard(rows)
}
