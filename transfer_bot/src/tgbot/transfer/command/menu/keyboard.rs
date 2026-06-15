// `/menu` 按钮布局。
// callback payload 协议放在 `callback` module，避免按钮布局和协议解析混在一起。

use crate::tgbot::send;

use super::super::super::store;
use super::super::common::{
    CommandStyle, balance_history_command, build_copy_only_row, build_refresh_return_menu_row,
    downloads_command,
};
use super::super::downloads::build_downloads_menu_callback_data;
use super::super::help;
use super::super::job::{
    build_job_pause_callback_data, build_job_resume_callback_data, build_job_status_callback_data,
    build_job_stop_callback_data,
};
use super::super::{build_cache_button_data, build_health_button_data};
use super::callback::{self, MenuPage};
use super::input::{MenuDraftSummary, MenuJobAction};

/// 构建当前菜单页按钮。
pub(super) fn build_menu_buttons(
    page: MenuPage,
    recent_jobs: &[store::JobProgressSnapshot],
    is_admin: bool,
    draft_summary: Option<&MenuDraftSummary>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    match page {
        MenuPage::Home => home_buttons(recent_jobs, is_admin, draft_summary),
        MenuPage::TasksHub => tasks_hub_buttons(recent_jobs),
        MenuPage::AccountHub => account_hub_buttons(is_admin),
        MenuPage::AdminHub if is_admin => admin_hub_buttons(),
        MenuPage::AdminHub => user_home_fallback_buttons(),
        MenuPage::Transfer => transfer_buttons(),
        MenuPage::Downloads => downloads_buttons(),
        MenuPage::Jobs => jobs_buttons(),
        MenuPage::Lookup => lookup_buttons(),
        MenuPage::Config if is_admin => config_buttons(),
        MenuPage::Config => user_home_fallback_buttons(),
        MenuPage::Help => help_buttons(is_admin),
    }
}

/// 首页按钮。
fn home_buttons(
    _recent_jobs: &[store::JobProgressSnapshot],
    is_admin: bool,
    draft_summary: Option<&MenuDraftSummary>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = vec![
        vec![
            send::build_callback_button(
                "开始转存",
                &callback::new_transfer_callback_data(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "快速转存",
                &callback::quick_transfer_default_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "快速查询",
                &callback::quick_lookup_default_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            menu_nav_button(
                "任务",
                MenuPage::TasksHub,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            menu_nav_button(
                "账户",
                MenuPage::AccountHub,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            menu_nav_button(
                "帮助",
                MenuPage::Help,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
    ];
    if let Some(draft) = draft_summary {
        rows.insert(
            0,
            vec![
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
            ],
        );
    }
    if is_admin {
        rows.insert(
            2,
            vec![menu_nav_button(
                "管理",
                MenuPage::AdminHub,
                tdlib_rs::enums::ButtonStyle::Default,
            )],
        );
    }
    rows.push(vec![
        menu_nav_button(
            "刷新",
            MenuPage::Home,
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        menu_nav_button(
            "帮助",
            MenuPage::Help,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]);
    rows
}

/// 任务 hub 按钮。
fn tasks_hub_buttons(
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
        vec![menu_nav_button(
            "结果查询",
            MenuPage::Lookup,
            tdlib_rs::enums::ButtonStyle::Default,
        )],
        build_refresh_return_menu_row(
            menu_nav_button(
                "刷新",
                MenuPage::TasksHub,
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
        build_copy_only_row(send::build_copy_button(
            "复制当前列表",
            &downloads_command(Some("all"), None, None, CommandStyle::Long),
            tdlib_rs::enums::ButtonStyle::Default,
        )),
    ];
    rows.extend(recent_job_buttons(recent_jobs));
    rows
}

/// 账户 hub 按钮。
fn account_hub_buttons(is_admin: bool) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = vec![
        vec![
            send::build_callback_button(
                "余额",
                &super::super::points::build_balance_home_callback_data(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "积分流水",
                &super::super::points::build_balance_history_home_callback_data(10, 1),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_refresh_return_menu_row(
            menu_nav_button(
                "刷新",
                MenuPage::AccountHub,
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
fn admin_hub_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
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
        build_refresh_return_menu_row(
            menu_nav_button(
                "刷新",
                MenuPage::AdminHub,
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

/// 转存页按钮。
fn transfer_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_callback_button(
                "开始转存",
                &callback::new_transfer_callback_data(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "快速转存",
                &callback::quick_transfer_default_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_refresh_return_menu_row(
            menu_nav_button(
                "刷新",
                MenuPage::Transfer,
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
        build_copy_only_row(send::build_copy_button(
            "复制取消命令",
            "/cancel",
            tdlib_rs::enums::ButtonStyle::Default,
        )),
    ]
}

/// 下载页按钮。
fn downloads_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            downloads_button("全部", "all", tdlib_rs::enums::ButtonStyle::Primary),
            downloads_button("运行", "run", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("等待", "wait", tdlib_rs::enums::ButtonStyle::Default),
        ],
        vec![
            downloads_button("下载", "dl", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("上传", "up", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("就绪", "ready", tdlib_rs::enums::ButtonStyle::Default),
        ],
        vec![
            downloads_button("完成", "done", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("成功", "ok", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("失败", "fail", tdlib_rs::enums::ButtonStyle::Default),
        ],
        vec![
            downloads_button("暂停", "pause", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button(
                "停止中",
                "cancelling",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            downloads_button("已停止", "cancel", tdlib_rs::enums::ButtonStyle::Default),
        ],
        build_refresh_return_menu_row(
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
        ),
    ]
}

/// 任务页按钮。
fn jobs_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            downloads_button("最近任务", "all", tdlib_rs::enums::ButtonStyle::Primary),
            downloads_button("运行任务", "run", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("暂停任务", "pause", tdlib_rs::enums::ButtonStyle::Default),
        ],
        vec![
            downloads_button(
                "停止中",
                "cancelling",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            downloads_button("已停止", "cancel", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("失败任务", "fail", tdlib_rs::enums::ButtonStyle::Default),
        ],
        vec![
            downloads_button("成功任务", "ok", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("就绪任务", "ready", tdlib_rs::enums::ButtonStyle::Default),
        ],
        vec![
            job_id_input_button(
                "输入详情",
                MenuJobAction::Status,
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            job_id_input_button(
                "输入暂停",
                MenuJobAction::Pause,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            job_id_input_button(
                "输入恢复",
                MenuJobAction::Resume,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            job_id_input_button(
                "输入停止",
                MenuJobAction::Stop,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_refresh_return_menu_row(
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
        ),
    ]
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

/// 配置页按钮。
fn config_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    super::super::config_cmd::build_config_buttons()
}

/// 首页最近任务快捷按钮。
///
/// 这些按钮只携带 job_id，并复用 `/job` 详情 callback；菜单不复制任务详情逻辑。
fn recent_job_buttons(
    recent_jobs: &[store::JobProgressSnapshot],
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    if recent_jobs.is_empty() {
        return Vec::new();
    }

    let mut rows = vec![vec![downloads_button(
        "查看最近任务",
        "all",
        tdlib_rs::enums::ButtonStyle::Primary,
    )]];
    let detail_rows = recent_jobs
        .iter()
        .take(5)
        .map(|snapshot| {
            let status = snapshot.job.status.as_str();
            let style = if matches!(
                status,
                store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING | store::JOB_STATUS_PAUSED
            ) {
                tdlib_rs::enums::ButtonStyle::Primary
            } else {
                tdlib_rs::enums::ButtonStyle::Default
            };
            let job_id = snapshot.job.id;
            let mut row = vec![send::build_callback_button(
                &format!("#{} {}", snapshot.job.id, snapshot.job.status),
                &build_job_status_callback_data(snapshot.job.id),
                style,
            )];
            row.extend(recent_job_control_buttons(job_id, status));
            row
        })
        .collect::<Vec<_>>();
    rows.extend(detail_rows);
    rows
}

/// 首页最近任务快捷控制。
///
/// 这里和 `/downloads` 一样只生成 `/job` callback，不直接改任务状态。
fn recent_job_control_buttons(
    job_id: i64,
    status: &str,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    if matches!(
        status,
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
    ) {
        return vec![
            send::build_callback_button(
                "暂停",
                &build_job_pause_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "停止",
                &build_job_stop_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ];
    }

    if status == store::JOB_STATUS_PAUSED {
        return vec![
            send::build_callback_button(
                "恢复",
                &build_job_resume_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "停止",
                &build_job_stop_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ];
    }

    Vec::new()
}

/// 帮助页按钮。
fn help_buttons(is_admin: bool) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = vec![
        vec![send::build_callback_button(
            "帮助目录",
            &help::build_help_callback_data(None),
            tdlib_rs::enums::ButtonStyle::Primary,
        )],
        vec![
            send::build_callback_button(
                "转存帮助",
                &help::build_help_callback_data(Some("transfer")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "查询帮助",
                &help::build_help_callback_data(Some("lookup")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "下载帮助",
                &help::build_help_callback_data(Some("downloads")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_callback_button(
                "任务帮助",
                &help::build_help_callback_data(Some("job")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "积分帮助",
                &help::build_help_callback_data(Some("points")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![send::build_callback_button(
            "菜单帮助",
            &help::build_help_callback_data(Some("menu")),
            tdlib_rs::enums::ButtonStyle::Default,
        )],
        vec![send::build_callback_button(
            "帮助说明",
            &help::build_help_callback_data(Some("help")),
            tdlib_rs::enums::ButtonStyle::Default,
        )],
        build_refresh_return_menu_row(
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
            menu_nav_button(
                "转存页",
                MenuPage::Transfer,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ),
    ];
    if is_admin {
        rows.insert(
            3,
            vec![send::build_callback_button(
                "配置帮助",
                &help::build_help_callback_data(Some("config")),
                tdlib_rs::enums::ButtonStyle::Default,
            )],
        );
    }
    rows
}

/// 普通用户误入 admin 配置页时只保留安全导航。
fn user_home_fallback_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        build_refresh_return_menu_row(
            menu_nav_button(
                "刷新",
                MenuPage::Home,
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            menu_nav_button(
                "帮助",
                MenuPage::Help,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            menu_nav_button(
                "返回",
                MenuPage::Config,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ),
        build_copy_only_row(send::build_copy_button(
            "复制余额命令",
            "/balance",
            tdlib_rs::enums::ButtonStyle::Default,
        )),
        build_copy_only_row(send::build_copy_button(
            "复制积分流水",
            &balance_history_command(10, 1, CommandStyle::Long),
            tdlib_rs::enums::ButtonStyle::Default,
        )),
    ]
}

/// 构建等待用户输入 job_id 的任务控制按钮。
fn job_id_input_button(
    text: &str,
    action: MenuJobAction,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(text, &callback::job_id_input_callback_data(action), style)
}

/// 构建下载筛选 callback 按钮。
fn downloads_button(
    text: &str,
    filter: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    if let Some(data) = build_downloads_menu_callback_data(filter, 8) {
        return send::build_callback_button(text, &data, style);
    }

    tracing::debug!(
        filter,
        "invalid downloads menu filter, fallback to copy command button"
    );
    send::build_copy_button(
        text,
        &downloads_command(Some(filter), None, None, CommandStyle::Short),
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
        let rows = build_menu_buttons(MenuPage::Home, &[], true, None);

        assert_eq!(rows[0][0].text, "开始转存");
        assert_eq!(rows[0][1].text, "快速转存");
        assert_eq!(rows[0][2].text, "快速查询");
        assert_eq!(rows[1][0].text, "任务");
        assert_eq!(rows[1][1].text, "账户");
        assert_eq!(rows[2][0].text, "管理");
    }

    // 首页按钮按“主要动作 -> hub 导航 -> footer”分组，避免再次变回功能总表。
    #[test]
    fn test_home_buttons_use_hub_navigation() {
        let rows = build_menu_buttons(MenuPage::Home, &[], false, None);
        assert_eq!(rows[1][0].text, "任务");
        assert_eq!(rows[1][1].text, "账户");
        assert_eq!(rows[1][2].text, "帮助");
    }

    // 下载按钮应直接复用 downloads callback，不让菜单重复实现分页逻辑。
    #[test]
    fn test_downloads_buttons_use_downloads_callback() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::Downloads, &[], true, None);

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

        let rows = build_menu_buttons(MenuPage::Help, &[], true, None);

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("help button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert!(decoded.starts_with("h:"));
    }

    // 首页存在最近任务时，最近任务入口仍应保留在首页底部，不因为 hub 化而丢失。
    #[test]
    fn test_recent_jobs_button_uses_downloads_callback() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(
            MenuPage::TasksHub,
            &[snapshot_with_status("running")],
            true,
            None,
        );
        let recent = rows
            .iter()
            .flatten()
            .find(|button| button.text == "查看最近任务")
            .expect("tasks hub should have recent jobs button");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &recent.r#type else {
            panic!("recent jobs button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert!(decoded.starts_with("d:"));
    }

    // 任务 hub 最近任务应能直接暂停/停止，减少进入详情页再操作的步骤。
    #[test]
    fn test_tasks_hub_recent_jobs_have_inline_controls() {
        let rows = build_menu_buttons(
            MenuPage::TasksHub,
            &[snapshot_with_status("running")],
            true,
            None,
        );
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"暂停"));
        assert!(labels.contains(&"停止"));
    }

    // 任务 hub 应承接原首页的状态快捷入口。
    #[test]
    fn test_tasks_hub_has_status_shortcuts() {
        let rows = build_menu_buttons(MenuPage::TasksHub, &[], true, None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"最近任务"));
        assert!(labels.contains(&"运行中"));
        assert!(labels.contains(&"已暂停"));
        assert!(labels.contains(&"失败/已停"));
    }

    // 首页按钮应收敛成高频动作 + hub + footer，不再直接暴露细页或最近任务。
    #[test]
    fn test_home_buttons_follow_row_hierarchy() {
        let rows = build_menu_buttons(MenuPage::Home, &[], true, None);

        assert_eq!(rows[0][0].text, "开始转存");
        assert_eq!(rows[1][0].text, "任务");
        assert_eq!(rows[1][1].text, "账户");
        assert_eq!(rows[2][0].text, "管理");
        assert_eq!(rows[3][0].text, "刷新");
        assert_eq!(rows[3][1].text, "帮助");
        assert_eq!(rows.len(), 4);
    }

    // 下载页和帮助页都应保留独立的刷新/返回/菜单层级。
    #[test]
    fn test_downloads_and_help_buttons_follow_footer_hierarchy() {
        let downloads = build_menu_buttons(MenuPage::Downloads, &[], true, None);
        let help = build_menu_buttons(MenuPage::Help, &[], true, None);

        assert_eq!(downloads[4][0].text, "刷新");
        assert_eq!(downloads[4][1].text, "首页");
        assert_eq!(downloads[4][2].text, "帮助");

        assert_eq!(help[5][0].text, "帮助说明");
        assert_eq!(help[6][0].text, "刷新");
        assert_eq!(help[6][1].text, "首页");
        assert_eq!(help[6][2].text, "转存页");
        assert!(
            !help
                .iter()
                .flatten()
                .any(|button| button.text == "复制帮助命令")
        );
    }

    // 管理 hub 应提供用户积分流水入口，避免手打 `/points history <user_id>`。
    #[test]
    fn test_admin_hub_has_point_ledger_input() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::AdminHub, &[], true, None);
        let button = rows
            .iter()
            .flatten()
            .find(|button| button.text == "用户流水")
            .expect("admin hub should have point ledger input");

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &button.r#type else {
            panic!("point ledger input must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "m:pl");
    }

    // 首页存在未完成输入时，应把恢复动作放在最前面，避免用户重新开始导致旧草稿残留。
    #[test]
    fn test_home_buttons_show_pending_input_shortcuts() {
        use base64::{Engine as _, engine::general_purpose};

        let draft = MenuDraftSummary {
            title: "快速转存"
        };
        let rows = build_menu_buttons(MenuPage::Home, &[], true, Some(&draft));

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
        let rows = build_menu_buttons(MenuPage::Downloads, &[], true, None);
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

    // 下载筛选参数未来调整时，菜单按钮不能因为旧参数 panic，应降级为复制命令按钮。
    #[test]
    fn test_downloads_button_invalid_filter_falls_back_to_copy() {
        let button = downloads_button("未知筛选", "unknown", tdlib_rs::enums::ButtonStyle::Default);

        assert!(matches!(
            button.r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::CopyText(_)
        ));
    }

    // 帮助菜单应覆盖所有 help topic，不让用户必须记 `/help <topic>`。
    #[test]
    fn test_help_buttons_cover_all_topics() {
        let rows = build_menu_buttons(MenuPage::Help, &[], true, None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        for expected in [
            "帮助目录",
            "转存帮助",
            "查询帮助",
            "下载帮助",
            "任务帮助",
            "积分帮助",
            "配置帮助",
            "菜单帮助",
            "帮助说明",
        ] {
            assert!(labels.contains(&expected), "missing help topic: {expected}");
        }
    }

    // 普通用户隐藏 admin-only 配置帮助时，不能误删同一行的菜单帮助。
    #[test]
    fn test_help_buttons_for_user_keep_menu_help_without_config() {
        let rows = build_menu_buttons(MenuPage::Help, &[], false, None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"菜单帮助"));
        assert!(!labels.contains(&"配置帮助"));
        assert!(!labels.contains(&"复制帮助命令"));
    }

    // 任务菜单应同时提供列表筛选、job_id 输入向导和复制模板。
    #[test]
    fn test_jobs_buttons_have_job_id_input_callbacks() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::Jobs, &[], true, None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        for expected in ["输入详情", "输入暂停", "输入恢复", "输入停止"] {
            assert!(labels.contains(&expected), "missing job input: {expected}");
        }

        let status_button = rows
            .iter()
            .flatten()
            .find(|button| button.text == "输入详情")
            .expect("status input button should exist");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &status_button.r#type
        else {
            panic!("job input button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "m:jst");
    }

    // 已经有交互入口的页面不应继续保留旧模板复制按钮，避免按钮区重复表达同一动作。
    #[test]
    fn test_menu_pages_drop_redundant_template_copy_buttons() {
        let transfer = build_menu_buttons(MenuPage::Transfer, &[], true, None);
        let downloads = build_menu_buttons(MenuPage::Downloads, &[], true, None);
        let jobs = build_menu_buttons(MenuPage::Jobs, &[], true, None);
        let lookup = build_menu_buttons(MenuPage::Lookup, &[], true, None);

        let labels = |rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>| {
            rows.into_iter()
                .flatten()
                .map(|button| button.text)
                .collect::<Vec<_>>()
        };

        let transfer_labels = labels(transfer);
        let downloads_labels = labels(downloads);
        let jobs_labels = labels(jobs);
        let lookup_labels = labels(lookup);

        assert!(!transfer_labels.contains(&"复制转存模板".to_owned()));
        assert!(transfer_labels.contains(&"复制取消命令".to_owned()));
        assert!(!transfer_labels.contains(&"指定目标".to_owned()));
        assert!(!transfer_labels.contains(&"默认目标".to_owned()));
        assert!(!downloads_labels.contains(&"复制全部列表".to_owned()));
        assert!(!downloads_labels.contains(&"复制运行列表命令".to_owned()));
        assert!(!jobs_labels.contains(&"复制详情模板".to_owned()));
        assert!(!jobs_labels.contains(&"复制停止模板".to_owned()));
        assert!(!lookup_labels.contains(&"复制查询模板".to_owned()));
    }

    // 首页 footer 不应重新膨胀成细页导航，只保留刷新和帮助。
    #[test]
    fn test_home_buttons_drop_self_home_footer_button() {
        let rows = build_menu_buttons(MenuPage::Home, &[], true, None);
        let footer = &rows[3];

        assert_eq!(footer[0].text, "刷新");
        assert_eq!(footer[1].text, "帮助");
        assert_eq!(footer.len(), 2);
    }

    // 任务 hub 应承接最近任务详情和列表复制入口。
    #[test]
    fn test_tasks_hub_recent_jobs_and_copy() {
        let rows = build_menu_buttons(
            MenuPage::TasksHub,
            &[snapshot_with_status("running")],
            true,
            None,
        );
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"查看最近任务"));
        assert!(labels.contains(&"#42 running"));
        assert!(labels.contains(&"复制当前列表"));
    }

    // 账户 hub 应提供余额和流水入口，admin 额外有用户流水。
    #[test]
    fn test_account_hub_buttons() {
        let user_rows = build_menu_buttons(MenuPage::AccountHub, &[], false, None);
        let admin_rows = build_menu_buttons(MenuPage::AccountHub, &[], true, None);

        assert_eq!(user_rows[0][0].text, "余额");
        assert_eq!(user_rows[0][1].text, "积分流水");
        assert!(
            !user_rows
                .iter()
                .flatten()
                .any(|button| button.text == "用户流水")
        );
        assert!(
            admin_rows
                .iter()
                .flatten()
                .any(|button| button.text == "用户流水")
        );
    }

    // 管理 hub 应集中承接配置、健康、缓存和用户流水，不再散落首页。
    #[test]
    fn test_admin_hub_buttons() {
        let rows = build_menu_buttons(MenuPage::AdminHub, &[], true, None);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        for expected in ["运行配置", "运行健康", "文件缓存", "用户流水"] {
            assert!(
                labels.contains(&expected),
                "missing admin hub button: {expected}"
            );
        }
    }

    // 普通用户误入 admin 配置页时不应同时出现两处“帮助”导航。
    #[test]
    fn test_user_home_fallback_buttons_drop_duplicate_help() {
        let rows = build_menu_buttons(MenuPage::Config, &[], false, None);
        let help_count = rows
            .iter()
            .flatten()
            .filter(|button| button.text == "帮助")
            .count();

        assert_eq!(help_count, 1);
        assert!(rows.iter().flatten().any(|button| button.text == "返回"));
    }

    // 转存页不应再保留与首行动作完全等价的重复按钮。
    #[test]
    fn test_transfer_buttons_drop_duplicate_action_labels() {
        let transfer = build_menu_buttons(MenuPage::Transfer, &[], true, None);
        let labels = transfer
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"开始转存"));
        assert!(labels.contains(&"快速转存"));
        assert!(!labels.contains(&"指定目标"));
        assert!(!labels.contains(&"默认目标"));
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
