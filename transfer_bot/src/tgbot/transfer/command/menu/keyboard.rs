//! `/menu` 交互菜单键盘布局模块。
//!
//! 负责聚合与构建各个菜单页面（首页、任务中心、管理中心、下载队列、任务列表、查重、帮助等）的内联按钮网格布局。
//! 底层回调载荷（callback payload）的编解码协议交由 `callback` 子模块处理，确保视图层布局与协议细节分离。

mod home;
mod hubs;
mod recent_jobs;

use crate::tgbot::send;

use super::super::super::store;
use super::super::common::build_refresh_return_menu_row;
use super::super::downloads::build_downloads_menu_filter_rows;
use super::super::help;
use super::super::job::build_job_menu_filter_rows;
use super::callback::{self, MenuPage};
use super::input::MenuDraftSummary;
use home::home_buttons;
use hubs::{admin_hub_buttons, tasks_hub_buttons};

/// 构建当前指定菜单页面的内联按钮（单元测试默认环境入口）。
///
/// 默认使用全局测试上下文并赋予超级管理员（owner = true）权限。
///
/// # 参数说明
/// - `page`: 目标菜单页面类型枚举
/// - `recent_jobs`: 最近任务快照列表切片
/// - `draft_summary`: 当前挂起的输入草稿摘要引用（若有）
///
/// # 返回值
/// 返回二维内联按钮向量矩阵。
#[cfg(test)]
pub(super) fn build_menu_buttons(
    page: MenuPage,
    recent_jobs: &[store::JobProgressSnapshot],
    draft_summary: Option<&MenuDraftSummary>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    build_menu_buttons_on(
        crate::app_context::app_context().as_ref(),
        page,
        recent_jobs,
        draft_summary,
        true,
    )
}

/// 构建当前指定菜单页面的内联按钮（单元测试可指定角色权限版本）。
///
/// # 参数说明
/// - `page`: 目标菜单页面类型枚举
/// - `recent_jobs`: 最近任务快照列表切片
/// - `draft_summary`: 当前挂起的输入草稿摘要引用（若有）
/// - `is_owner`: 是否具备超级管理员权限
///
/// # 返回值
/// 返回二维内联按钮向量矩阵。
#[cfg(test)]
fn build_menu_buttons_for_actor(
    page: MenuPage,
    recent_jobs: &[store::JobProgressSnapshot],
    draft_summary: Option<&MenuDraftSummary>,
    is_owner: bool,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    build_menu_buttons_on(
        crate::app_context::app_context().as_ref(),
        page,
        recent_jobs,
        draft_summary,
        is_owner,
    )
}

/// 构建当前指定菜单页面内联按钮的核心上下文实现函数。
///
/// 根据传入的 `MenuPage` 分发到对应的具体子页面或外部子模块生成按钮网格。
///
/// # 页面分支说明
/// - `MenuPage::Home`: 首页卡片按钮（快捷转存/继续输入、中心枢纽入口、刷新）
/// - `MenuPage::TasksHub`: 任务中心（快捷筛选、最近 5 任务状态与控制、通用底部）
/// - `MenuPage::AdminHub`: 管理中心（配置项、健康检查、缓存、账号授权等）
/// - `MenuPage::Downloads`: 下载队列过滤页（运行中、排队中、暂停、失败等状态过滤按钮）
/// - `MenuPage::Jobs`: 任务列表细页按钮
/// - `MenuPage::Lookup`: 查重检索页按钮（快速查询、指定目标查询）
/// - `MenuPage::Config`: 运行时全局配置面板按钮（委托给 `config_cmd` 模块）
/// - `MenuPage::Targets`: 转存目标管理面板按钮（委托给 `targets` 模块）
/// - `MenuPage::Help`: 帮助中心主题分类按钮
///
/// # 参数说明
/// - `app`: 全局应用上下文引用
/// - `page`: 目标菜单页面枚举
/// - `recent_jobs`: 最近任务快照切片
/// - `draft_summary`: 当前输入会话草稿摘要（若有）
/// - `is_owner`: 是否为超级管理员
///
/// # 返回值
/// 返回针对 Telegram 消息内联键盘的二维按钮列表。
pub(super) fn build_menu_buttons_on(
    app: &crate::app_context::AppContext,
    page: MenuPage,
    recent_jobs: &[store::JobProgressSnapshot],
    draft_summary: Option<&MenuDraftSummary>,
    is_owner: bool,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    match page {
        // 主菜单首页
        MenuPage::Home => home_buttons(recent_jobs, draft_summary),
        // 任务中心二级枢纽
        MenuPage::TasksHub => tasks_hub_buttons(recent_jobs),
        // 管理中心二级枢纽
        MenuPage::AdminHub => admin_hub_buttons(is_owner),
        // 下载队列过滤页面
        MenuPage::Downloads => downloads_buttons(),
        // 任务列表细页
        MenuPage::Jobs => jobs_buttons(),
        // 查重功能页面
        MenuPage::Lookup => lookup_buttons(),
        // 系统运行参数配置页面
        MenuPage::Config => super::super::config_cmd::build_config_buttons_on(app),
        // 目标会话管理页面
        MenuPage::Targets => super::super::targets::build_targets_buttons_on(app),
        // 帮助与文档页面
        MenuPage::Help => help_buttons(),
    }
}

/// 构建“下载队列”（Downloads）菜单页面的内联按钮。
///
/// 结构包括：
/// 1. 顶部状态筛选网格（全部、运行、等待、下载中、上传中、完成、失败、暂停、停止等）；
/// 2. 底部控制栏（刷新当前下载页、返回主菜单首页、查看指令列表）。
fn downloads_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = build_downloads_menu_filter_rows();
    rows.push(build_refresh_return_menu_row(
        menu_nav_button(
            "刷新",
            MenuPage::Downloads,
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        menu_nav_button(
            "首页",
            MenuPage::Home,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        view_commands_button(),
    ));
    rows
}

/// 构建“任务列表”（Jobs）菜单页面的内联按钮。
///
/// 结构包括：
/// 1. 任务过滤行（最近任务、运行中、已暂停、已失败等）；
/// 2. 底部控制栏（刷新当前任务页、返回主菜单首页、查看指令列表）。
fn jobs_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = build_job_menu_filter_rows();
    rows.push(build_refresh_return_menu_row(
        menu_nav_button(
            "刷新",
            MenuPage::Jobs,
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        menu_nav_button(
            "首页",
            MenuPage::Home,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        view_commands_button(),
    ));
    rows
}

/// 构建“查重检索”（Lookup）菜单页面的内联按钮。
///
/// 结构包括：
/// 1. 首行核心查重操作：
///    - “快速查询”（Primary 样式）：以配置的默认目标进行消息查重；
///    - “指定目标”（Default 样式）：手动输入目标会话以进行消息查重；
/// 2. 底部控制栏（刷新当前查重页、返回主菜单首页、查看指令列表）。
fn lookup_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_callback_button(
                "快速查询",
                &callback::quick_lookup_default_callback_data(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "指定目标",
                &callback::new_lookup_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_refresh_return_menu_row(
            menu_nav_button(
                "刷新",
                MenuPage::Lookup,
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            menu_nav_button(
                "首页",
                MenuPage::Home,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            view_commands_button(),
        ),
    ]
}

/// 构建通用的“查看命令”内联按钮。
///
/// 点击后触发打开全局指令帮助中心卡片，日常菜单页面不直接展示冗长命令说明，以保持界面整洁。
fn view_commands_button() -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        "查看命令",
        &super::super::build_help_button_data(None),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}

/// 构建“帮助中心”（Help）菜单页面的内联按钮。
///
/// 结构包括：
/// 1. 帮助主题分类入口（转存、查询、下载列表、任务控制、运行配置、目标管理等）；
/// 2. 底部控制栏（刷新帮助页、返回首页、直接发起“开始转存”）。
fn help_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = help::build_help_menu_topic_rows();
    rows.push(build_refresh_return_menu_row(
        menu_nav_button(
            "刷新",
            MenuPage::Help,
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        menu_nav_button(
            "首页",
            MenuPage::Home,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_callback_button(
            "开始转存",
            &callback::new_transfer_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ));
    rows
}

/// 构建带有下载状态筛选载荷的 Telegram 回调按钮。
///
/// # 参数说明
/// - `text`: 按钮显示的文本内容
/// - `filter`: 状态筛选名称（如 "all", "running", "paused" 等）
/// - `limit`: 分页数量上限
/// - `style`: 按钮高亮或常规视觉样式
fn downloads_button(
    text: &str,
    filter: &str,
    limit: u64,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        text,
        &crate::tgbot::transfer::command::require_downloads_filter_button_data(filter, limit),
        style,
    )
}

/// 构建菜单内页面导航跳转按钮。
///
/// # 参数说明
/// - `text`: 按钮上显示的标签文本
/// - `page`: 目标页面枚举类型
/// - `style`: 按钮样式
fn menu_nav_button(
    text: &str,
    page: MenuPage,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(text, &callback::menu_page_callback_data(page), style)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试主菜单首页按钮结构：
    /// 首页应提供核心动作入口（快速转存、指定目标），二级枢纽入口（任务、管理、查看命令），
    /// 以及刷新控制行，确保操作路径足够精简且不膨胀为繁复的总表。
    #[test]
    fn test_home_buttons() {
        let rows = build_menu_buttons(MenuPage::Home, &[], None);

        // 第一行：高频核心转存操作
        assert_eq!(rows[0][0].text, "快速转存");
        assert_eq!(rows[0][1].text, "指定目标");
        // 第二行：二级中心 Hub 导航与命令说明入口
        assert_eq!(rows[1][0].text, "任务");
        assert_eq!(rows[1][1].text, "管理");
        assert_eq!(rows[1][2].text, "查看命令");
        // 快速查询已收敛至任务 Hub，首页不直接展示
        assert!(
            !rows
                .iter()
                .flatten()
                .any(|button| button.text == "快速查询")
        );
        // 第三行：单独一行刷新按钮
        assert_eq!(rows[2][0].text, "刷新");
        assert_eq!(rows[2].len(), 1);
    }

    /// 测试首页按钮的分组层级：
    /// 遵循“主要动作 -> hub 导航 -> footer”分组规范，避免界面信息过载。
    #[test]
    fn test_home_buttons_use_hub_navigation() {
        let rows = build_menu_buttons(MenuPage::Home, &[], None);
        assert_eq!(rows[1][0].text, "任务");
        assert_eq!(rows[1][1].text, "管理");
        assert_eq!(rows[1][2].text, "查看命令");
        assert_eq!(rows[2][0].text, "刷新");
    }

    /// 测试下载页面按钮的回调载荷协议：
    /// 下载按钮应直接复用 downloads 模块回调格式（d: 前缀），避免菜单重复维护下载分页与筛选逻辑。
    #[test]
    fn test_downloads_buttons_use_downloads_callback() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::Downloads, &[], None);

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("downloads button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        // 验证载荷以 "d:" 为前缀
        assert!(decoded.starts_with("d:"));
    }

    /// 测试帮助页面按钮的回调载荷协议：
    /// 帮助按钮应直接走 help 回调格式（h: 前缀），点击后即时展示主题说明，无需手动输入命令。
    #[test]
    fn test_help_buttons_use_help_callback() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::Help, &[], None);

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("help button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        // 验证载荷以 "h:" 为前缀
        assert!(decoded.starts_with("h:"));
    }

    /// 测试任务中心中“最近任务”按钮的回调载荷协议：
    /// 存在最近任务时，入口按钮继续复用 downloads 模块的筛选回调。
    #[test]
    fn test_recent_jobs_button_uses_downloads_callback() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::TasksHub, &[snapshot_with_status("running")], None);
        let recent = rows
            .iter()
            .flatten()
            .find(|button| button.text == "最近任务")
            .expect("tasks hub should have recent jobs button");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &recent.r#type else {
            panic!("recent jobs button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert!(decoded.starts_with("d:"));
    }

    /// 测试任务中心行内快捷控制按钮行为：
    /// 运行中的任务行应直接提供“暂停”以及导向停止二次确认页的“停止”按钮（Danger 样式）。
    #[test]
    fn test_tasks_hub_recent_jobs_have_inline_controls() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::TasksHub, &[snapshot_with_status("running")], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        // 验证包含行内控制动作
        assert!(labels.contains(&"暂停"));
        assert!(labels.contains(&"停止"));
        let stop = rows
            .iter()
            .flatten()
            .find(|button| button.text == "停止")
            .expect("tasks hub should have stop confirmation button");
        // 停止按钮应具有 Danger 红色告警视觉样式
        assert_eq!(stop.style, tdlib_rs::enums::ButtonStyle::Danger);
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &stop.r#type else {
            panic!("stop button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        // 停止确认回调协议："j:sc:<id>"
        assert_eq!(decoded, "j:sc:42");
    }

    /// 测试任务中心继承的状态快捷入口集合：
    /// 任务 Hub 应承接原本分散在首页的任务状态快捷筛选与查重操作。
    #[test]
    fn test_tasks_hub_has_status_shortcuts() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::TasksHub, &[], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        // 应包含的筛选与查重按钮
        assert!(labels.contains(&"最近任务"));
        assert!(labels.contains(&"运行中"));
        assert!(labels.contains(&"已暂停"));
        assert!(labels.contains(&"失败任务"));
        assert!(labels.contains(&"快速查询"));
        assert!(labels.contains(&"指定目标"));
        assert!(labels.contains(&"更多状态"));
        // 避免出现重复或已废弃的旧文案
        assert!(!labels.contains(&"下载列表"));
        assert!(!labels.contains(&"查询页"));
        assert!(!labels.contains(&"复制当前列表"));
        assert!(!labels.contains(&"查看最近任务"));

        let specified_lookup = rows
            .iter()
            .flatten()
            .find(|button| button.text == "指定目标")
            .expect("tasks hub should have specified lookup button");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) =
            &specified_lookup.r#type
        else {
            panic!("specified lookup must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "m:qlk");
    }

    /// 测试首页按钮行严格遵守三层层级：
    /// 第 1 行核心动作、第 2 行 Hub 枢纽、第 3 行刷新 footer，总共 3 行。
    #[test]
    fn test_home_buttons_follow_row_hierarchy() {
        let rows = build_menu_buttons(MenuPage::Home, &[], None);

        assert_eq!(rows[0][0].text, "快速转存");
        assert_eq!(rows[1][0].text, "任务");
        assert_eq!(rows[1][1].text, "管理");
        assert_eq!(rows[1][2].text, "查看命令");
        assert_eq!(rows[2][0].text, "刷新");
        assert_eq!(rows[2].len(), 1);
        assert_eq!(rows.len(), 3);
        assert!(
            !rows
                .iter()
                .flatten()
                .any(|button| button.text == "查看最近任务")
        );
        assert!(
            !rows
                .iter()
                .flatten()
                .any(|button| button.text == "复制当前列表")
        );
    }

    /// 测试下载页与帮助页底部导航层级：
    /// 均统一遵循独立的三按钮 footer 结构（刷新当前页、返回首页、功能动作/查看命令）。
    #[test]
    fn test_downloads_and_help_buttons_follow_footer_hierarchy() {
        use base64::Engine as _;

        let downloads = build_menu_buttons(MenuPage::Downloads, &[], None);
        let help = build_menu_buttons(MenuPage::Help, &[], None);

        // 下载页 footer：刷新 -> 首页 -> 查看命令
        assert_eq!(downloads[4][0].text, "刷新");
        assert_eq!(downloads[4][1].text, "首页");
        assert_eq!(downloads[4][2].text, "查看命令");

        // 帮助页 footer：刷新 -> 首页 -> 开始转存
        assert_eq!(help[4][0].text, "刷新");
        assert_eq!(help[4][1].text, "首页");
        assert_eq!(help[4][2].text, "开始转存");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &help[4][2].r#type
        else {
            panic!("help transfer action must be callback");
        };
        let decoded = String::from_utf8(
            base64::engine::general_purpose::STANDARD
                .decode(&callback.data)
                .expect("callback should be base64"),
        )
        .expect("callback should be utf8");
        // 开始转存对应 "m:new" 回调载荷
        assert_eq!(decoded, "m:new");
        assert!(
            !help
                .iter()
                .flatten()
                .any(|button| button.text == "复制帮助命令")
        );
    }

    /// 测试挂起输入草稿时的首页按钮：
    /// 当用户有未完成的输入流程时，首页首行应替换为“继续输入：{title}”与“取消输入”，防止输入草稿孤立遗留。
    #[test]
    fn test_home_buttons_show_pending_input_shortcuts() {
        use base64::{Engine as _, engine::general_purpose};

        let draft = MenuDraftSummary {
            title: "快速转存"
        };
        let rows = build_menu_buttons(MenuPage::Home, &[], Some(&draft));

        // 首行显示继续和取消按钮
        assert_eq!(rows[0][0].text, "继续输入：快速转存");
        assert_eq!(rows[0][1].text, "取消输入");
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();
        // 普通的快速转存与指定目标入口暂时被替换
        assert!(!labels.contains(&"快速转存"));
        assert!(!labels.contains(&"指定目标"));

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("continue input button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        // 继续输入回调协议："m:ci"
        assert_eq!(decoded, "m:ci");
    }

    /// 测试下载筛选菜单覆盖全部任务过滤状态选项。
    #[test]
    fn test_downloads_buttons_cover_all_filters() {
        let rows = build_menu_buttons(MenuPage::Downloads, &[], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        for expected in [
            "全部",
            "运行",
            "等待",
            "下载",
            "上传",
            "就绪",
            "完成",
            "成功",
            "失败",
            "暂停",
            "停止中",
            "已停止",
        ] {
            assert!(
                labels.contains(&expected),
                "missing downloads filter: {expected}"
            );
        }
    }

    /// 测试帮助中心覆盖所有关键主题分类按钮，避免用户需要记忆命令。
    #[test]
    fn test_help_buttons_cover_all_topics() {
        let rows = build_menu_buttons(MenuPage::Help, &[], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        for expected in [
            "转存",
            "查询",
            "下载列表",
            "任务控制",
            "交互菜单",
            "运行健康",
            "文件缓存",
            "运行配置",
            "目标配置",
            "授权管理",
        ] {
            assert!(labels.contains(&expected), "missing help topic: {expected}");
        }
    }

    /// 测试帮助菜单中包含管理类主题，且不再展示冗余的文本复制按钮。
    #[test]
    fn test_help_buttons_include_management_topics() {
        let rows = build_menu_buttons(MenuPage::Help, &[], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"转存"));
        assert!(labels.contains(&"交互菜单"));
        assert!(labels.contains(&"运行健康"));
        assert!(labels.contains(&"运行配置"));
        assert!(labels.contains(&"目标配置"));
        assert!(!labels.contains(&"复制帮助命令"));
    }

    /// 测试任务菜单优先使用可交互点击列表，而非要求用户记忆输入 job_id。
    #[test]
    fn test_jobs_buttons_prefer_clickable_lists_over_job_id_input() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::Jobs, &[], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        // 不应再包含旧式的文本输入按钮
        for removed in ["输入详情", "输入暂停", "输入恢复", "输入停止"] {
            assert!(
                !labels.contains(&removed),
                "unexpected job input: {removed}"
            );
        }
        // 应包含状态列表入口
        assert!(labels.contains(&"最近任务"));
        assert!(labels.contains(&"运行任务"));
        assert!(labels.contains(&"暂停任务"));

        let recent_button = rows
            .iter()
            .flatten()
            .find(|button| button.text == "最近任务")
            .expect("recent jobs button should exist");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &recent_button.r#type
        else {
            panic!("recent jobs button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert!(decoded.starts_with("d:"));
    }

    /// 测试页面中去除冗余的旧文本命令复制按钮（已由内联交互卡片替代）。
    #[test]
    fn test_menu_pages_drop_redundant_template_copy_buttons() {
        let downloads = build_menu_buttons(MenuPage::Downloads, &[], None);
        let jobs = build_menu_buttons(MenuPage::Jobs, &[], None);
        let lookup = build_menu_buttons(MenuPage::Lookup, &[], None);

        let labels = |rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>| {
            rows.into_iter()
                .flatten()
                .map(|button| button.text)
                .collect::<Vec<_>>()
        };

        let downloads_labels = labels(downloads);
        let jobs_labels = labels(jobs);
        let lookup_labels = labels(lookup);

        assert!(!downloads_labels.contains(&"复制全部列表".to_owned()));
        assert!(!downloads_labels.contains(&"复制运行列表命令".to_owned()));
        assert!(!jobs_labels.contains(&"复制详情模板".to_owned()));
        assert!(!jobs_labels.contains(&"复制停止模板".to_owned()));
        assert!(!lookup_labels.contains(&"复制查询模板".to_owned()));
    }

    /// 测试主页 footer 保持精简，只保留单个刷新按钮，不添加冗余导航。
    #[test]
    fn test_home_buttons_drop_self_home_footer_button() {
        let rows = build_menu_buttons(MenuPage::Home, &[], None);
        let footer = &rows[2];

        assert_eq!(footer[0].text, "刷新");
        assert_eq!(footer.len(), 1);
    }

    /// 测试任务 Hub 展示最近任务详情，避免重复挂载“查看最近任务”等同义按钮。
    #[test]
    fn test_tasks_hub_recent_jobs_and_copy() {
        let rows = build_menu_buttons(MenuPage::TasksHub, &[snapshot_with_status("running")], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"最近任务"));
        assert!(labels.contains(&"#42 running"));
        assert!(!labels.contains(&"查看最近任务"));
        assert!(!labels.contains(&"复制当前列表"));
    }

    /// 测试两个二级中心 Hub（任务 Hub 与管理 Hub）的底部导航层级保持严格一致。
    #[test]
    fn test_hub_footers_use_same_hierarchy() {
        let tasks = build_menu_buttons(MenuPage::TasksHub, &[], None);
        let admin = build_menu_buttons(MenuPage::AdminHub, &[], None);

        for footer in [
            tasks.last().expect("tasks hub should have footer"),
            admin.last().expect("admin hub should have footer"),
        ] {
            assert_eq!(footer[0].text, "刷新");
            assert_eq!(footer[1].text, "首页");
            assert_eq!(footer[2].text, "查看命令");
        }
    }

    /// 测试首页仅保留一个统一的“查看命令”入口。
    #[test]
    fn test_home_buttons_have_single_command_entry() {
        let rows = build_menu_buttons(MenuPage::Home, &[], None);
        let command_count = rows
            .iter()
            .flatten()
            .filter(|button| button.text == "查看命令")
            .count();

        assert_eq!(command_count, 1);
    }

    /// 测试存在任务快照时，任务 Hub 剔除同义的“查看最近任务”按钮。
    #[test]
    fn test_tasks_hub_drops_duplicate_recent_list_entry_when_jobs_exist() {
        let rows = build_menu_buttons(MenuPage::TasksHub, &[snapshot_with_status("running")], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"最近任务"));
        assert!(!labels.contains(&"查看最近任务"));
        assert!(!labels.contains(&"复制当前列表"));
    }

    /// 测试管理中心二级 Hub 按钮布局与权限控制：
    /// 超级管理员展示授权管理入口，普通操作者则隐藏该特权入口。
    #[test]
    fn test_admin_hub_buttons() {
        use base64::{Engine as _, engine::general_purpose};

        // 超级管理员视图
        let rows = build_menu_buttons_for_actor(MenuPage::AdminHub, &[], None, true);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        for expected in ["运行配置", "目标配置", "运行健康", "文件缓存", "授权管理"]
        {
            assert!(
                labels.contains(&expected),
                "missing admin hub button: {expected}"
            );
        }

        // 非管理员视图：不应展示“授权管理”
        let non_owner_rows = build_menu_buttons_for_actor(MenuPage::AdminHub, &[], None, false);
        assert!(
            !non_owner_rows
                .iter()
                .flatten()
                .any(|button| button.text == "授权管理")
        );

        // 验证授权管理按钮回调协议为 "au:refresh"
        let auth_button = rows
            .iter()
            .flatten()
            .find(|button| button.text == "授权管理")
            .expect("missing authorization management button");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &auth_button.r#type
        else {
            panic!("authorization management should use callback");
        };
        let decoded = String::from_utf8(
            general_purpose::STANDARD
                .decode(&callback.data)
                .expect("callback should be base64"),
        )
        .expect("callback should be utf8");
        assert_eq!(decoded, "au:refresh");

        // 验证目标配置按钮导航到 "m:tg"
        let button = rows
            .iter()
            .flatten()
            .find(|button| button.text == "目标配置")
            .expect("missing target config button");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &button.r#type else {
            panic!("target config should navigate by callback");
        };
        let decoded = String::from_utf8(
            general_purpose::STANDARD
                .decode(&callback.data)
                .expect("callback should be base64"),
        )
        .expect("callback should be utf8");
        assert_eq!(decoded, "m:tg");
    }

    /// 测试目标管理运行界面中的命令按钮：
    /// 确保复用了 targets 命令模块的标准按钮，而非在管理页机械复制命令行。
    #[test]
    fn test_runtime_admin_pages_have_command_buttons() {
        let targets = build_menu_buttons(MenuPage::Targets, &[], None);

        assert_eq!(targets[0][0].text, "默认目标");
        assert!(
            targets
                .iter()
                .flatten()
                .any(|button| button.text == "默认目标")
        );
        assert!(
            targets
                .iter()
                .flatten()
                .any(|button| button.text == "恢复私聊默认")
        );
        assert!(
            !targets
                .iter()
                .flatten()
                .any(|button| button.text == "设默认")
        );
        assert!(
            !targets
                .iter()
                .flatten()
                .any(|button| button.text == "设路由")
        );
        assert!(
            !targets
                .iter()
                .flatten()
                .any(|button| button.text == "设别名")
        );
    }

    /// 辅助测试函数：构造指定状态的任务进度快照对象。
    fn snapshot_with_status(status: &str) -> store::JobProgressSnapshot {
        let now = store::now_utc8();
        store::JobProgressSnapshot {
            job: store::JobProgressJob {
                id: 42,
                target_chat_id: -100,
                result_message_link: None,
                status: status.to_owned(),
                total_items: 1,
                last_error: None,
                created_at: now,
                updated_at: now,
            },
            pending_count: 0,
            preparing_count: 0,
            prepared_count: 0,
            uploading_count: 0,
            success_count: 0,
            failed_count: 0,
            cancelled_count: 0,
            active_download_files: 0,
            active_downloaded_bytes: 0,
            active_download_total_bytes: 0,
            has_unknown_download_total: false,
            active_upload_files: 0,
            active_uploaded_bytes: 0,
            active_upload_total_bytes: 0,
            has_unknown_upload_total: false,
        }
    }
}
