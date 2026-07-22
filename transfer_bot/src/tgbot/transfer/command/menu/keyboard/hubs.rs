// `/menu` 二级 hub 按钮。
// hub 负责把同一领域的入口收在一起，细页只保留具体操作。

use crate::tgbot::send;

use super::super::super::super::store;
use super::super::super::common::build_refresh_return_menu_row;
use super::super::super::{build_cache_button_data, build_health_button_data};
use super::super::{HubEntryAction, HubEntrySpec, admin_hub_specs, tasks_hub_specs};
use super::recent_jobs::recent_job_buttons;
use super::{MenuPage, callback, downloads_button, menu_nav_button};

/// 任务 hub 按钮。
pub(super) fn tasks_hub_buttons(
    recent_jobs: &[store::JobProgressSnapshot],
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = build_hub_button_rows(tasks_hub_specs());
    if !recent_jobs.is_empty() {
        rows.extend(recent_job_buttons(recent_jobs));
    }
    rows.push(hub_footer(MenuPage::TasksHub));
    rows
}

/// 管理 hub 按钮。
pub(super) fn admin_hub_buttons(is_owner: bool) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = build_hub_button_rows(admin_hub_specs(is_owner));
    rows.push(hub_footer(MenuPage::AdminHub));
    rows
}

/// 三个 hub 使用同一套 footer：刷新当前 hub，回首页，再看帮助。
fn hub_footer(page: MenuPage) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    build_refresh_return_menu_row(
        menu_nav_button("刷新", page, tdlib_rs::enums::ButtonStyle::Primary),
        menu_nav_button(
            "首页",
            MenuPage::Home,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        super::view_commands_button(),
    )
}

/// 把共享 hub 入口定义转换成按钮行。
///
/// hub 页只消费 `menu.rs` 中央维护的入口元数据，避免按钮标题和命令预览各改各的。
fn build_hub_button_rows(
    rows: Vec<Vec<HubEntrySpec>>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    rows.into_iter()
        .map(|row| row.iter().map(build_hub_button).collect())
        .collect()
}

/// 根据共享入口定义构建单个 hub 按钮。
fn build_hub_button(spec: &HubEntrySpec) -> tdlib_rs::types::InlineKeyboardButton {
    match spec.action {
        HubEntryAction::DownloadsFilter { filter, limit } => {
            downloads_button(spec.text, filter, limit, spec.style.clone())
        }
        HubEntryAction::MenuPage(page) => menu_nav_button(spec.text, page, spec.style.clone()),
        HubEntryAction::QuickLookupDefault => send::build_callback_button(
            spec.text,
            &callback::quick_lookup_default_callback_data(),
            spec.style.clone(),
        ),
        HubEntryAction::NewLookup => send::build_callback_button(
            spec.text,
            &callback::new_lookup_callback_data(),
            spec.style.clone(),
        ),
        HubEntryAction::HealthHome => {
            send::build_callback_button(spec.text, &build_health_button_data(), spec.style.clone())
        }
        HubEntryAction::CacheHome => {
            send::build_callback_button(spec.text, &build_cache_button_data(), spec.style.clone())
        }
        HubEntryAction::AuthHome => send::build_callback_button(
            spec.text,
            &super::super::super::auth::build_auth_panel_callback_data(),
            spec.style.clone(),
        ),
    }
}
