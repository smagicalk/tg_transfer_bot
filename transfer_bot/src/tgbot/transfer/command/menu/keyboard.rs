// `/menu` 按钮布局。
// callback payload 协议放在 `callback` module，避免按钮布局和协议解析混在一起。

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

/// 构建当前菜单页按钮。
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
    )
}

/// 构建当前菜单页按钮的上下文版本。
pub(super) fn build_menu_buttons_on(
    app: &crate::app_context::AppContext,
    page: MenuPage,
    recent_jobs: &[store::JobProgressSnapshot],
    draft_summary: Option<&MenuDraftSummary>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    match page {
        MenuPage::Home => home_buttons(recent_jobs, draft_summary),
        MenuPage::TasksHub => tasks_hub_buttons(recent_jobs),
        MenuPage::AdminHub => admin_hub_buttons(),
        MenuPage::Downloads => downloads_buttons(),
        MenuPage::Jobs => jobs_buttons(),
        MenuPage::Lookup => lookup_buttons(),
        MenuPage::Config => super::super::config_cmd::build_config_buttons_on(app),
        MenuPage::Targets => super::super::targets::build_targets_buttons_on(app),
        MenuPage::Help => help_buttons(),
    }
}

/// 下载页按钮。
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
        menu_nav_button(
            "帮助",
            MenuPage::Help,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ));
    rows
}

/// 任务页按钮。
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
        menu_nav_button(
            "帮助",
            MenuPage::Help,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ));
    rows
}

/// 查询页按钮。
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
            menu_nav_button(
                "帮助",
                MenuPage::Help,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ),
    ]
}

/// 帮助页按钮。
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

/// 构建下载筛选 callback 按钮。
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

/// 构建菜单导航按钮。
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

    // 首页应提供主要入口，保持日常操作足够短。
    #[test]
    fn test_home_buttons() {
        let rows = build_menu_buttons(MenuPage::Home, &[], None);

        assert_eq!(rows[0][0].text, "快速转存");
        assert_eq!(rows[0][1].text, "指定目标");
        assert_eq!(rows[1][0].text, "任务");
        assert_eq!(rows[1][1].text, "管理");
        assert_eq!(rows[1][2].text, "帮助");
        assert!(
            !rows
                .iter()
                .flatten()
                .any(|button| button.text == "快速查询")
        );
        assert_eq!(rows[2][0].text, "刷新");
        assert_eq!(rows[2].len(), 1);
    }

    // 首页按钮按“主要动作 -> hub 导航 -> footer”分组，避免再次变回功能总表。
    #[test]
    fn test_home_buttons_use_hub_navigation() {
        let rows = build_menu_buttons(MenuPage::Home, &[], None);
        assert_eq!(rows[1][0].text, "任务");
        assert_eq!(rows[1][1].text, "管理");
        assert_eq!(rows[1][2].text, "帮助");
        assert_eq!(rows[2][0].text, "刷新");
    }

    // 下载按钮应直接复用 downloads callback，不让菜单重复实现分页逻辑。
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
        assert!(decoded.starts_with("d:"));
    }

    // 帮助页按钮应直接走 help callback，不需要用户复制命令再发送。
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
        assert!(decoded.starts_with("h:"));
    }

    // 有最近任务时，列表入口应继续复用“最近任务”筛选 callback。
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

    // 任务 hub 最近任务应能直接暂停，并把停止导向确认页。
    #[test]
    fn test_tasks_hub_recent_jobs_have_inline_controls() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::TasksHub, &[snapshot_with_status("running")], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"暂停"));
        assert!(labels.contains(&"停止"));
        let stop = rows
            .iter()
            .flatten()
            .find(|button| button.text == "停止")
            .expect("tasks hub should have stop confirmation button");
        assert_eq!(stop.style, tdlib_rs::enums::ButtonStyle::Danger);
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &stop.r#type else {
            panic!("stop button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "j:sc:42");
    }

    // 任务 hub 应承接原首页的状态快捷入口。
    #[test]
    fn test_tasks_hub_has_status_shortcuts() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::TasksHub, &[], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"最近任务"));
        assert!(labels.contains(&"运行中"));
        assert!(labels.contains(&"已暂停"));
        assert!(labels.contains(&"失败任务"));
        assert!(labels.contains(&"快速查询"));
        assert!(labels.contains(&"指定目标"));
        assert!(labels.contains(&"更多状态"));
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

    // 首页按钮应收敛成高频动作 + hub + footer，不再直接暴露细页或最近任务。
    #[test]
    fn test_home_buttons_follow_row_hierarchy() {
        let rows = build_menu_buttons(MenuPage::Home, &[], None);

        assert_eq!(rows[0][0].text, "快速转存");
        assert_eq!(rows[1][0].text, "任务");
        assert_eq!(rows[1][1].text, "管理");
        assert_eq!(rows[1][2].text, "帮助");
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

    // 下载页和帮助页都应保留独立的刷新/返回/菜单层级。
    #[test]
    fn test_downloads_and_help_buttons_follow_footer_hierarchy() {
        use base64::Engine as _;

        let downloads = build_menu_buttons(MenuPage::Downloads, &[], None);
        let help = build_menu_buttons(MenuPage::Help, &[], None);

        assert_eq!(downloads[4][0].text, "刷新");
        assert_eq!(downloads[4][1].text, "首页");
        assert_eq!(downloads[4][2].text, "帮助");

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
        assert_eq!(decoded, "m:new");
        assert!(
            !help
                .iter()
                .flatten()
                .any(|button| button.text == "复制帮助命令")
        );
    }

    // 首页存在未完成输入时，应把恢复动作放在最前面，避免用户重新开始导致旧草稿残留。
    #[test]
    fn test_home_buttons_show_pending_input_shortcuts() {
        use base64::{Engine as _, engine::general_purpose};

        let draft = MenuDraftSummary {
            title: "快速转存"
        };
        let rows = build_menu_buttons(MenuPage::Home, &[], Some(&draft));

        assert_eq!(rows[0][0].text, "继续输入：快速转存");
        assert_eq!(rows[0][1].text, "取消输入");

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("continue input button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "m:ci");
    }

    // 下载菜单应覆盖 `/downloads` 当前支持的全部筛选参数。
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

    // 帮助菜单应直接复用 help topic 导航，不让用户必须记 `/help <topic>`。
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
        ] {
            assert!(labels.contains(&expected), "missing help topic: {expected}");
        }
    }

    // 单所有者帮助页展示全部保留主题。
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

    // 任务菜单应通过列表选中任务，不再要求手动输入 job_id。
    #[test]
    fn test_jobs_buttons_prefer_clickable_lists_over_job_id_input() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::Jobs, &[], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        for removed in ["输入详情", "输入暂停", "输入恢复", "输入停止"] {
            assert!(
                !labels.contains(&removed),
                "unexpected job input: {removed}"
            );
        }
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

    // 已经有交互入口的页面不应继续保留旧模板复制按钮，避免按钮区重复表达同一动作。
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

    // 首页 footer 不应重新膨胀成细页导航，只保留刷新。
    #[test]
    fn test_home_buttons_drop_self_home_footer_button() {
        let rows = build_menu_buttons(MenuPage::Home, &[], None);
        let footer = &rows[2];

        assert_eq!(footer[0].text, "刷新");
        assert_eq!(footer.len(), 1);
    }

    // 任务 hub 有最近任务时应展示任务详情，不再额外挂同义列表入口和复制入口。
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

    // 两个 hub 的 footer 统一为“刷新当前页 -> 首页 -> 帮助”，减少跨页记忆成本。
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
            assert_eq!(footer[2].text, "帮助");
        }
    }

    // 首页不应同时出现两个“帮助”按钮。
    #[test]
    fn test_home_buttons_have_single_help_entry() {
        let rows = build_menu_buttons(MenuPage::Home, &[], None);
        let help_count = rows
            .iter()
            .flatten()
            .filter(|button| button.text == "帮助")
            .count();

        assert_eq!(help_count, 1);
    }

    // 有最近任务时，任务 hub 不应同时出现“最近任务”和“查看最近任务”两个列表入口。
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

    // 管理 hub 应集中承接配置、健康和缓存，不再散落首页。
    #[test]
    fn test_admin_hub_buttons() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::AdminHub, &[], None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        for expected in ["运行配置", "目标配置", "运行健康", "文件缓存"] {
            assert!(
                labels.contains(&expected),
                "missing admin hub button: {expected}"
            );
        }

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

    // 目标配置页应复用目标命令模块按钮，而不是只在管理页复制命令。
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

    fn snapshot_with_status(status: &str) -> store::JobProgressSnapshot {
        let now = store::now_utc8();
        store::JobProgressSnapshot {
            job: store::JobProgressJob {
                id: 42,
                target_chat_id: -100,
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
        }
    }
}
