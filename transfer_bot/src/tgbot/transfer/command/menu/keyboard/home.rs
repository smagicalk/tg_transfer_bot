// `/menu` 首页按钮。
// 首页只保留高频动作和 hub 入口，避免再次变成功能总表。

use crate::tgbot::send;

use super::{MenuDraftSummary, MenuPage, callback, menu_nav_button};

/// 首页按钮。
pub(super) fn home_buttons(
    _recent_jobs: &[crate::tgbot::transfer::store::JobProgressSnapshot],
    draft_summary: Option<&MenuDraftSummary>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = Vec::new();
    if let Some(draft) = draft_summary {
        rows.push(vec![
            send::build_callback_button(
                &format!("继续输入：{}", draft.title),
                &callback::continue_input_callback_data(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "取消输入",
                &callback::cancel_input_callback_data(),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
        ]);
    } else {
        rows.push(vec![
            send::build_callback_button(
                "快速转存",
                &callback::quick_transfer_default_callback_data(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "指定目标",
                &callback::new_transfer_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ]);
    }
    rows.push(vec![
        menu_nav_button(
            "任务",
            MenuPage::TasksHub,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        menu_nav_button(
            "管理",
            MenuPage::AdminHub,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        super::view_commands_button(),
    ]);
    rows.push(vec![menu_nav_button(
        "刷新",
        MenuPage::Home,
        tdlib_rs::enums::ButtonStyle::Primary,
    )]);
    rows
}
