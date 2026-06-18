// `/menu` 二级 hub 按钮。
// hub 负责把同一领域的入口收在一起，细页只保留具体操作。

use crate::tgbot::send;

use super::super::super::super::store;
use super::super::super::common::{
    CommandStyle, build_copy_only_row, build_refresh_return_menu_row, downloads_command,
};
use super::super::super::{build_cache_button_data, build_health_button_data};
use super::recent_jobs::recent_job_buttons;
use super::{MenuPage, callback, downloads_button, menu_nav_button};

/// 任务 hub 按钮。
pub(super) fn tasks_hub_buttons(
    recent_jobs: &[store::JobProgressSnapshot],
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = vec![
        vec![
            downloads_button("最近任务", "all", tdlib_rs::enums::ButtonStyle::Primary),
            downloads_button("运行中", "run", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("已暂停", "pause", tdlib_rs::enums::ButtonStyle::Default),
        ],
        vec![
            downloads_button("失败/已停", "fail", tdlib_rs::enums::ButtonStyle::Default),
            menu_nav_button(
                "下载列表",
                MenuPage::Downloads,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            menu_nav_button(
                "任务控制",
                MenuPage::Jobs,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_callback_button(
                "快速查询",
                &callback::quick_lookup_default_callback_data(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            menu_nav_button(
                "查询页",
                MenuPage::Lookup,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
    ];
    let has_recent_jobs = !recent_jobs.is_empty();
    if has_recent_jobs {
        rows.extend(recent_job_buttons(recent_jobs));
    } else {
        rows.push(build_copy_only_row(send::build_copy_button(
            "复制当前列表",
            &downloads_command(Some("all"), None, None, CommandStyle::Long),
            tdlib_rs::enums::ButtonStyle::Default,
        )));
    }
    rows.push(hub_footer(MenuPage::TasksHub));
    rows
}

/// 账户 hub 按钮。
pub(super) fn account_hub_buttons(
    is_admin: bool,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = vec![
        vec![
            send::build_callback_button(
                "余额",
                &super::super::super::points::build_balance_home_callback_data(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "积分流水",
                &super::super::super::points::build_balance_history_home_callback_data(10, 1),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        hub_footer(MenuPage::AccountHub),
    ];
    if is_admin {
        rows.insert(
            1,
            vec![send::build_callback_button(
                "用户流水",
                &callback::point_ledger_user_input_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            )],
        );
    }
    rows
}

/// 管理 hub 按钮。
pub(super) fn admin_hub_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            menu_nav_button(
                "运行配置",
                MenuPage::Config,
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "运行健康",
                &build_health_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            menu_nav_button(
                "目标配置",
                MenuPage::Targets,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            menu_nav_button(
                "访问控制",
                MenuPage::Acl,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            menu_nav_button(
                "计费配置",
                MenuPage::Billing,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "文件缓存",
                &build_cache_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "用户流水",
                &callback::point_ledger_user_input_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        hub_footer(MenuPage::AdminHub),
    ]
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
        menu_nav_button(
            "帮助",
            MenuPage::Help,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    )
}
