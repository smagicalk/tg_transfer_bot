// 转存进度面板的按钮构造。
// 这里只生成 Telegram inline keyboard，具体发送/编辑由上层 progress 模块负责。

use crate::tgbot::transfer::command::common::{
    CommandStyle, downloads_command as build_downloads_command, job_command as build_job_command,
    lookup_command as build_lookup_command, transfer_command as build_transfer_command,
};
use crate::tgbot::transfer::command::{
    build_downloads_filter_button_data, build_downloads_status_button_data,
    build_job_pause_button_data, build_job_resume_button_data, build_job_status_button_data,
    build_job_stop_button_data,
};
use crate::tgbot::transfer::store;

/// 构造进度面板按钮。
pub(super) fn build_transfer_progress_keyboard(
    job_id: Option<i64>,
    job_status: Option<&str>,
    source_link: &str,
    target_chat_id: i64,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let lookup_command = build_lookup_command(source_link, target_chat_id, CommandStyle::Short);
    let list_status = job_status
        .map(list_status_for_job_status)
        .unwrap_or(store::JOB_STATUS_RUNNING);
    let mut rows = vec![vec![
        crate::tgbot::send::build_callback_button(
            list_label_for_job_status(job_status),
            &build_downloads_status_button_data(list_status, 8),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制查询命令",
            &lookup_command,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]];

    if let Some(job_id) = job_id {
        rows.extend(build_job_control_rows(job_id, job_status));
    }

    crate::tgbot::send::build_inline_keyboard(rows)
}

/// 按任务状态构造可点击控制按钮和可复制命令。
///
/// 进度面板可能被最终结果复用，因此这里不能对 cancelled/cancelling 再展示暂停按钮。
fn build_job_control_rows(
    job_id: i64,
    job_status: Option<&str>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut action_row = vec![crate::tgbot::send::build_callback_button(
        "查看任务详情",
        &build_job_status_button_data(job_id),
        tdlib_rs::enums::ButtonStyle::Primary,
    )];
    let mut copy_row = Vec::new();

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
            copy_row.push(crate::tgbot::send::build_copy_button(
                "复制恢复命令",
                &build_job_command("r", job_id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ));
            copy_row.push(crate::tgbot::send::build_copy_button(
                "复制停止命令",
                &build_job_command("s", job_id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ));
        }
        Some(store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING)
        | Some(store::JOB_STATUS_CANCELLED) => {
            copy_row.push(crate::tgbot::send::build_copy_button(
                "复制 job_id",
                &job_id.to_string(),
                tdlib_rs::enums::ButtonStyle::Default,
            ));
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
            copy_row.push(crate::tgbot::send::build_copy_button(
                "复制暂停命令",
                &build_job_command("p", job_id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ));
            copy_row.push(crate::tgbot::send::build_copy_button(
                "复制停止命令",
                &build_job_command("s", job_id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ));
        }
    }

    let mut rows = vec![action_row];
    if !copy_row.is_empty() {
        rows.push(copy_row);
    }
    rows
}

/// 任务状态到列表筛选的映射，保证面板顶部按钮跳到最相关列表。
fn list_status_for_job_status(status: &str) -> &str {
    match status {
        store::JOB_STATUS_PAUSED => store::JOB_STATUS_PAUSED,
        store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING => {
            store::JOB_STATUS_CANCELLING
        }
        store::JOB_STATUS_CANCELLED => store::JOB_STATUS_CANCELLED,
        _ => store::JOB_STATUS_RUNNING,
    }
}

/// 根据任务状态生成列表按钮文案。
fn list_label_for_job_status(status: Option<&str>) -> &'static str {
    match status {
        Some(store::JOB_STATUS_PAUSED) => "查看暂停列表",
        Some(store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING) => "查看停止列表",
        Some(store::JOB_STATUS_CANCELLED) => "查看已停列表",
        _ => "查看运行列表",
    }
}

/// 构造最终结果按钮。
pub(super) fn build_transfer_result_keyboard(
    source_link: &str,
    target_chat_id: i64,
    job_id: Option<i64>,
    result_link: Option<&str>,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let lookup_command = build_lookup_command(source_link, target_chat_id, CommandStyle::Short);
    let retry_command = build_transfer_command(source_link, target_chat_id, CommandStyle::Short);
    let mut first_row = Vec::new();
    if let Some(result_link) = result_link {
        // 只有 TDLib 返回或本模块兜底生成的 HTTP(S) 链接才放“打开转存消息”按钮；客户端 deeplink 不稳定，
        // 放到 URL 按钮里会造成点击无反应。
        if crate::tgbot::send::is_openable_url(result_link) {
            first_row.push(crate::tgbot::send::build_url_button(
                "打开转存消息",
                result_link,
                tdlib_rs::enums::ButtonStyle::Primary,
            ));
        }
        first_row.push(crate::tgbot::send::build_copy_button(
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
    }
    if let Some(job_id) = job_id {
        first_row.push(crate::tgbot::send::build_callback_button(
            "查看任务详情",
            &build_job_status_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Default,
        ));
    }
    first_row.push(crate::tgbot::send::build_copy_button(
        "复制查询命令",
        &lookup_command,
        tdlib_rs::enums::ButtonStyle::Default,
    ));

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
    let mut second_row = vec![crate::tgbot::send::build_copy_button(
        "复制重新转存",
        &retry_command,
        tdlib_rs::enums::ButtonStyle::Default,
    )];
    if let Some(callback_data) = build_downloads_filter_button_data(list_filter, 8) {
        second_row.push(crate::tgbot::send::build_callback_button(
            list_label,
            &callback_data,
            tdlib_rs::enums::ButtonStyle::Primary,
        ));
    }
    second_row.push(crate::tgbot::send::build_copy_button(
        "复制列表命令",
        &build_downloads_command(Some(list_filter), None, None, CommandStyle::Short),
        tdlib_rs::enums::ButtonStyle::Default,
    ));

    crate::tgbot::send::build_inline_keyboard(vec![first_row, second_row])
}
