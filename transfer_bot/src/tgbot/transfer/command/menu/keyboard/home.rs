//! `/menu` 主菜单首页内联按钮构建模块。
//!
//! 首页采用精简卡片设计，仅保留最高频的核心动作（如快速转存、继续未完成输入）
//! 以及二级中心枢纽（Hub）导航入口，避免首页充斥过多低频功能而造成视觉负担。

use crate::tgbot::send;

use super::{MenuDraftSummary, MenuPage, callback, menu_nav_button};

/// 构建主菜单首页（Home）的内联按钮网格布局。
///
/// 布局结构设计：
/// 1. 第一行（动态核心动作区）：
///    - 若用户存在未完成的输入草稿（`draft_summary.is_some()`），展示“继续输入：{title}”及“取消输入”按钮；
///    - 若无草稿，则展示“快速转存”（使用默认目标）与“指定目标”（手动指定新转存任务）快捷入口。
/// 2. 第二行（二级中心枢纽入口）：
///    - “任务”中心（`MenuPage::TasksHub`）：浏览和控制所有任务；
///    - “管理”中心（`MenuPage::AdminHub`）：系统配置与目标管理；
///    - “指令”按钮：展示快捷指令列表。
/// 3. 第三行（控制区）：
///    - “刷新”按钮：拉取最新数据并重新渲染主菜单首页。
///
/// # 参数说明
/// - `_recent_jobs`: 最近任务的进度快照列表切片（备用参数，当前首页紧凑模式下不直接生成行内单任务按钮）
/// - `draft_summary`: 用户当前挂起的输入会话摘要信息（若有）
///
/// # 返回值
/// 返回二维向量，外层代表按钮行（Row），内层代表该行中的具体内联按钮（InlineKeyboardButton）。
pub(super) fn home_buttons(
    _recent_jobs: &[crate::tgbot::transfer::store::JobProgressSnapshot],
    draft_summary: Option<&MenuDraftSummary>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = Vec::new();

    // 1. 首行：根据是否存在挂起的输入草稿动态渲染
    if let Some(draft) = draft_summary {
        // 用户处于某一未完成的输入流程中，显示继续与取消按钮
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
        // 无挂起输入，显示最常用的转存动作入口
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

    // 2. 第二行：二级中心枢纽导航入口与指令清单
    rows.push(vec![
        // 任务中心导航
        menu_nav_button(
            "任务",
            MenuPage::TasksHub,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        // 管理中心导航
        menu_nav_button(
            "管理",
            MenuPage::AdminHub,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        // 快捷命令帮助弹窗或说明
        super::view_commands_button(),
    ]);

    // 3. 第三行：刷新当前首页卡片状态
    rows.push(vec![menu_nav_button(
        "刷新",
        MenuPage::Home,
        tdlib_rs::enums::ButtonStyle::Primary,
    )]);

    rows
}
