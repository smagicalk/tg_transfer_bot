//! `/menu` 二级中心枢纽（Hub）按钮构建模块。
//!
//! 中心枢纽页负责将同一业务领域（例如任务管理领域、系统管理领域）的所有功能入口集中在一起，
//! 各个子页面只承载具体的操作卡片。本模块通过共享入口元数据规格（`HubEntrySpec`）来驱动按钮的统一样式生成。

use crate::tgbot::send;

use super::super::super::super::store;
use super::super::super::common::build_refresh_return_menu_row;
use super::super::super::{build_cache_button_data, build_health_button_data};
use super::super::{HubEntryAction, HubEntrySpec, admin_hub_specs, tasks_hub_specs};
use super::recent_jobs::recent_job_buttons;
use super::{MenuPage, callback, downloads_button, menu_nav_button};

/// 构建“任务中心”（TasksHub）的内联按钮列表。
///
/// 界面布局：
/// 1. 任务相关的各类操作入口（查重、下载队列过滤等，由 `tasks_hub_specs` 规格定义）；
/// 2. 最近 5 个任务的实时状态行与控制按钮（若有）；
/// 3. 标准 Hub 底部导航栏（刷新当前页、返回首页、查看指令帮助）。
///
/// # 参数说明
/// - `recent_jobs`: 最近任务的进度快照列表切片
///
/// # 返回值
/// 返回二维按钮矩阵。
pub(super) fn tasks_hub_buttons(
    recent_jobs: &[store::JobProgressSnapshot],
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    // 根据任务 Hub 入口规格构建基础按钮行
    let mut rows = build_hub_button_rows(tasks_hub_specs());
    // 若存在最近任务快照，则追加行内任务状态与控制按钮
    if !recent_jobs.is_empty() {
        rows.extend(recent_job_buttons(recent_jobs));
    }
    // 追加统一的 Hub 底部导航栏
    rows.push(hub_footer(MenuPage::TasksHub));
    rows
}

/// 构建“管理中心”（AdminHub）的内联按钮列表。
///
/// 界面布局：
/// 1. 管理功能操作入口（如目标设置、系统配置、健康检查、缓存管理、账号认证等）；
/// 2. 标准 Hub 底部导航栏（刷新当前页、返回首页、查看指令帮助）。
///
/// # 参数说明
/// - `is_owner`: 是否为超级管理员/Bot 拥有者（根据权限决定是否展示最高特权管理入口）
///
/// # 返回值
/// 返回二维按钮矩阵。
pub(super) fn admin_hub_buttons(is_owner: bool) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    // 根据权限动态筛选并构建管理 Hub 的按钮行
    let mut rows = build_hub_button_rows(admin_hub_specs(is_owner));
    // 追加统一的 Hub 底部导航栏
    rows.push(hub_footer(MenuPage::AdminHub));
    rows
}

/// 构建中心枢纽（Hub）通用的底部导航按钮行。
///
/// 导航行由三个按钮构成：
/// - “刷新”（Primary 样式）：重新拉取并渲染当前 Hub 页面数据；
/// - “首页”（Default 样式）：返回主菜单首页；
/// - “指令”按钮：弹窗或查看命令帮助清单。
///
/// # 参数说明
/// - `page`: 当前所在 Hub 的菜单页面枚举
///
/// # 返回值
/// 返回包含 3 个导航按钮的一维向量行。
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

/// 将共享的 Hub 入口规格二维矩阵批量转换为 Telegram 内联按钮二维矩阵。
///
/// 各 Hub 统一消费 `menu.rs` 中集中维护的 `HubEntrySpec` 元数据，
/// 避免按钮标题文案、回调动作和说明预览各处维护导致不一致。
///
/// # 参数说明
/// - `rows`: 入口规格二维向量
///
/// # 返回值
/// 返回内联按钮二维向量。
fn build_hub_button_rows(
    rows: Vec<Vec<HubEntrySpec>>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    rows.into_iter()
        .map(|row| row.iter().map(build_hub_button).collect())
        .collect()
}

/// 根据单个共享入口规格（`HubEntrySpec`）构建具体的 Telegram 内联按钮。
///
/// 根据入口动作类别（`HubEntryAction`）分发到对应的回调构建函数：
/// - `DownloadsFilter`: 跳转到下载队列过滤视图；
/// - `MenuPage`: 菜单内二级页面跳转；
/// - `QuickLookupDefault`: 触发默认配置快速查重；
/// - `NewLookup`: 触发手动指定链接查重输入；
/// - `HealthHome`: 打开系统健康度面板；
/// - `CacheHome`: 打开缓存管理面板；
/// - `AuthHome`: 打开登录与账号认证面板；
/// - `ExecutorHome`: 打开执行器（Worker）管理面板。
///
/// # 参数说明
/// - `spec`: 单个入口元数据规格引用
///
/// # 返回值
/// 返回已绑订回调数据与样式的 `InlineKeyboardButton`。
fn build_hub_button(spec: &HubEntrySpec) -> tdlib_rs::types::InlineKeyboardButton {
    match spec.action {
        // 下载列表过滤动作按钮
        HubEntryAction::DownloadsFilter { filter, limit } => {
            downloads_button(spec.text, filter, limit, spec.style.clone())
        }
        // 内部页面跳转导航按钮
        HubEntryAction::MenuPage(page) => menu_nav_button(spec.text, page, spec.style.clone()),
        // 使用默认目标快速查重
        HubEntryAction::QuickLookupDefault => send::build_callback_button(
            spec.text,
            &callback::quick_lookup_default_callback_data(),
            spec.style.clone(),
        ),
        // 新建查重输入
        HubEntryAction::NewLookup => send::build_callback_button(
            spec.text,
            &callback::new_lookup_callback_data(),
            spec.style.clone(),
        ),
        // 系统健康度面板
        HubEntryAction::HealthHome => {
            send::build_callback_button(spec.text, &build_health_button_data(), spec.style.clone())
        }
        // 缓存管理面板
        HubEntryAction::CacheHome => {
            send::build_callback_button(spec.text, &build_cache_button_data(), spec.style.clone())
        }
        // 账号认证授权面板
        HubEntryAction::AuthHome => send::build_callback_button(
            spec.text,
            &super::super::super::auth::build_auth_panel_callback_data(),
            spec.style.clone(),
        ),
        // 执行器监控管理面板
        HubEntryAction::ExecutorHome => crate::tgbot::executor::build_executor_panel_button(),
    }
}
