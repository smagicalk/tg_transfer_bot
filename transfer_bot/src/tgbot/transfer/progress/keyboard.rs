// 转存进度面板的按钮构造。
// 这里只生成 Telegram inline keyboard，具体发送/编辑由上层 progress 模块负责。

use crate::tgbot::transfer::command::{
    build_downloads_status_button_data, build_job_list_button_meta, build_menu_home_button_data,
    build_view_commands_button,
};
use crate::tgbot::transfer::store;

/// 构造转存进度面板的内联键盘。
///
/// 包括：任务交互控制行（若已有 job_id：暂停/恢复/停止/详情）以及底部通用导航行（查看列表/查看命令/菜单）。
///
/// # 参数
/// - `job_id`: 可选的关联任务 ID
/// - `job_status`: 可选的任务状态字符串（如 "running", "paused" 等）
/// - `_source_link`: 转存源链接（占位）
/// - `_target_chat_id`: 目标聊天 ID（占位）
///
/// # 返回
/// 组装完毕的 `ReplyMarkupInlineKeyboard` 内联键盘对象
pub(super) fn build_transfer_progress_keyboard(
    job_id: Option<i64>,
    job_status: Option<&str>,
    _source_link: &str,
    _target_chat_id: i64,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    // 根据任务状态决定底部列表按钮的目标状态与文案
    let (list_status, list_label) = job_status
        .map(build_job_list_button_meta)
        .unwrap_or(("run", "查看运行列表"));
    // 构造底部的通用导航操作行：列表跳转、查看命令帮助、返回主菜单
    let navigation_row = vec![
        crate::tgbot::send::build_callback_button(
            list_label,
            &build_downloads_status_button_data(list_status, 8),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        build_view_commands_button(Some(if job_id.is_some() { "job" } else { "transfer" })),
        crate::tgbot::send::build_callback_button(
            "菜单",
            &build_menu_home_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ];

    let mut rows = Vec::new();
    // 若已创建了真实任务，挂载对应的任务控制按钮行（暂停/恢复/停止/详情）
    if let Some(job_id) = job_id {
        rows.extend(build_job_control_rows(job_id, job_status));
    }
    // 添加导航按钮行
    rows.push(navigation_row);

    crate::tgbot::send::build_inline_keyboard(rows)
}

/// 按任务状态构造可点击控制按钮。
///
/// 进度面板可能被最终结果复用，因此这里不能对 cancelled/cancelling 再展示暂停按钮。
/// 按钮区只保留真正的交互控制；命令说明由导航行按需打开。
///
/// # 参数
/// - `job_id`: 任务 ID
/// - `job_status`: 任务当前状态字符串
///
/// # 返回
/// 任务控制内联按钮行数组
fn build_job_control_rows(
    job_id: i64,
    job_status: Option<&str>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    // 归一化任务状态标签
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

/// 构造转存终态结果面板的内联键盘。
///
/// # 参数
/// - `_source_link`: 转存源链接（占位）
/// - `_target_chat_id`: 目标聊天会话 ID（占位）
/// - `job_id`: 可选的任务 ID
/// - `result_link`: 可选的结果消息直达链接
///
/// # 返回
/// 结果面板内联键盘
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

    // 区分成功或失败结果，添加对应的列表导航行
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
