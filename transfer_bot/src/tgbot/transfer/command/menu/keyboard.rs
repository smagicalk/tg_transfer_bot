// `/menu` 按钮和 callback payload。
// payload 保持短格式，避免超过 Telegram callback_data 长度限制。

use crate::tgbot::send;

use super::super::super::store;
use super::super::common::{CommandStyle, config_show_command, lookup_command};
use super::super::downloads::build_downloads_menu_callback_data;
use super::super::help;
use super::super::job::build_job_status_callback_data;

/// 菜单按钮回调前缀。
const MENU_CALLBACK_PREFIX: &str = "m:";

/// 菜单页。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MenuPage {
    Home,
    Transfer,
    Downloads,
    Jobs,
    Lookup,
    Config,
    Help,
}

impl MenuPage {
    /// 页面标题，用于 callback 提示和文本标题。
    pub(super) fn title(self) -> &'static str {
        match self {
            Self::Home => "菜单",
            Self::Transfer => "转存",
            Self::Downloads => "下载",
            Self::Jobs => "任务",
            Self::Lookup => "查询",
            Self::Config => "配置",
            Self::Help => "帮助",
        }
    }

    /// 页面短编码，写进 callback payload。
    fn code(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Transfer => "t",
            Self::Downloads => "d",
            Self::Jobs => "j",
            Self::Lookup => "lk",
            Self::Config => "cfg",
            Self::Help => "h",
        }
    }

    /// 从 callback 短编码解析页面。
    fn parse(code: &str) -> Option<Self> {
        match code {
            "home" => Some(Self::Home),
            "t" => Some(Self::Transfer),
            "d" => Some(Self::Downloads),
            "j" => Some(Self::Jobs),
            "lk" => Some(Self::Lookup),
            "cfg" => Some(Self::Config),
            "h" => Some(Self::Help),
            _ => None,
        }
    }
}

/// 菜单 callback 动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MenuRequestAction {
    Page(MenuPage),
    NewTransfer,
    QuickTransferDefault,
    NewLookup,
    QuickLookupDefault,
}

/// 判断 callback payload 是否属于 `/menu`。
pub(super) fn is_menu_callback_data(data: &str) -> bool {
    data.starts_with(MENU_CALLBACK_PREFIX)
}

/// 解析菜单 callback payload。
pub(super) fn parse_menu_callback_data(data: &str) -> Option<MenuRequestAction> {
    let payload = data.strip_prefix(MENU_CALLBACK_PREFIX)?;
    match payload {
        "new" => Some(MenuRequestAction::NewTransfer),
        "qtd" => Some(MenuRequestAction::QuickTransferDefault),
        "qlk" => Some(MenuRequestAction::NewLookup),
        "qld" => Some(MenuRequestAction::QuickLookupDefault),
        page_code => MenuPage::parse(page_code).map(MenuRequestAction::Page),
    }
}

/// 构建当前菜单页按钮。
pub(super) fn build_menu_buttons(
    page: MenuPage,
    recent_jobs: &[store::JobProgressSnapshot],
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    match page {
        MenuPage::Home => home_buttons(recent_jobs),
        MenuPage::Transfer => transfer_buttons(),
        MenuPage::Downloads => downloads_buttons(),
        MenuPage::Jobs => jobs_buttons(),
        MenuPage::Lookup => lookup_buttons(),
        MenuPage::Config => config_buttons(),
        MenuPage::Help => help_buttons(),
    }
}

/// 首页按钮。
fn home_buttons(
    recent_jobs: &[store::JobProgressSnapshot],
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = vec![
        vec![
            menu_nav_button(
                "转存",
                MenuPage::Transfer,
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            menu_nav_button(
                "下载",
                MenuPage::Downloads,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            menu_nav_button(
                "任务",
                MenuPage::Jobs,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            menu_nav_button(
                "查询",
                MenuPage::Lookup,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            menu_nav_button(
                "配置",
                MenuPage::Config,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            menu_nav_button(
                "帮助",
                MenuPage::Help,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![send::build_copy_button(
            "复制 /m",
            "/m",
            tdlib_rs::enums::ButtonStyle::Default,
        )],
    ];
    rows.extend(recent_job_buttons(recent_jobs));
    rows
}

/// 转存页按钮。
fn transfer_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_callback_button(
                "快速转存",
                &menu_callback_data("qtd"),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "指定目标",
                &menu_callback_data("new"),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_copy_button("复制 /t", "/t ", tdlib_rs::enums::ButtonStyle::Default),
            send::build_copy_button(
                "复制完整模板",
                "/t <link> <target_chat_id>",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button("复制取消", "/cancel", tdlib_rs::enums::ButtonStyle::Default),
        ],
        footer_buttons(MenuPage::Transfer),
    ]
}

/// 下载页按钮。
fn downloads_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            downloads_button("全部", "all", tdlib_rs::enums::ButtonStyle::Primary),
            downloads_button("运行", "run", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("下载", "dl", tdlib_rs::enums::ButtonStyle::Default),
        ],
        vec![
            downloads_button("完成", "done", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("失败", "fail", tdlib_rs::enums::ButtonStyle::Default),
            downloads_button("暂停", "pause", tdlib_rs::enums::ButtonStyle::Default),
        ],
        vec![
            send::build_copy_button("复制 /d", "/d", tdlib_rs::enums::ButtonStyle::Default),
            send::build_copy_button(
                "复制 /d run",
                "/d run",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        footer_buttons(MenuPage::Downloads),
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
            send::build_copy_button(
                "详情模板",
                "/j st <job_id>",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "停止模板",
                "/j s <job_id>",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        footer_buttons(MenuPage::Jobs),
    ]
}

/// 查询页按钮。
fn lookup_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_callback_button(
                "快速查询",
                &menu_callback_data("qld"),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "指定目标",
                &menu_callback_data("qlk"),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_copy_button("复制 /lk", "/lk ", tdlib_rs::enums::ButtonStyle::Default),
            send::build_copy_button(
                "复制模板",
                &lookup_command("<link>", 0, CommandStyle::Short)
                    .replace(" 0", " <target_chat_id>"),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        footer_buttons(MenuPage::Lookup),
    ]
}

/// 配置页按钮。
fn config_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = super::super::config_cmd::build_config_buttons();
    rows.push(footer_buttons(MenuPage::Config));
    rows
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

    let mut rows = vec![vec![send::build_copy_button(
        "最近任务",
        "/d",
        tdlib_rs::enums::ButtonStyle::Primary,
    )]];
    let detail_buttons = recent_jobs
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
            send::build_callback_button(
                &format!("详情 #{} {}", snapshot.job.id, snapshot.job.status),
                &build_job_status_callback_data(snapshot.job.id),
                style,
            )
        })
        .collect::<Vec<_>>();
    rows.extend(detail_buttons.chunks(2).map(<[_]>::to_vec));
    rows
}

/// 旧版配置复制按钮。
///
/// 留给测试或回退排查使用；菜单和 `/cfg` 当前都走 `config_cmd::build_config_buttons`。
#[allow(dead_code)]
fn legacy_config_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_copy_button(
                "复制查看配置",
                &config_show_command(CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "并发=2",
                "/cfg set job_concurrency 2",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_copy_button(
                "延迟=2m",
                "/cfg set file_delete_delay_minutes 2",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "GC=30s",
                "/cfg set file_gc_interval_seconds 30",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        footer_buttons(MenuPage::Config),
    ]
}

/// 帮助页按钮。
fn help_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_callback_button(
                "帮助目录",
                &help::build_help_callback_data(None),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "转存帮助",
                &help::build_help_callback_data(Some("transfer")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "任务帮助",
                &help::build_help_callback_data(Some("job")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_copy_button("复制 /h", "/h", tdlib_rs::enums::ButtonStyle::Default),
            send::build_copy_button(
                "复制 /h transfer",
                "/h transfer",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        footer_buttons(MenuPage::Help),
    ]
}

/// 构建下载筛选 callback 按钮。
fn downloads_button(
    text: &str,
    filter: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    let data = build_downloads_menu_callback_data(filter, 8).unwrap_or_else(|| "/d".to_owned());
    send::build_callback_button(text, &data, style)
}

/// 构建统一页脚按钮。
///
/// 第一版没有多级历史栈，“返回”统一回首页，“刷新”刷新当前页。
fn footer_buttons(current_page: MenuPage) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![
        menu_nav_button(
            "首页",
            MenuPage::Home,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        menu_nav_button(
            "返回",
            MenuPage::Home,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        menu_nav_button("刷新", current_page, tdlib_rs::enums::ButtonStyle::Default),
    ]
}

/// 构建菜单导航按钮。
fn menu_nav_button(
    text: &str,
    page: MenuPage,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(text, &menu_callback_data(page.code()), style)
}

/// 生成菜单 callback payload。
fn menu_callback_data(action: &str) -> String {
    format!("{}{}", MENU_CALLBACK_PREFIX, action)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 菜单 callback 数据应能区分页面切换和新建转存动作。
    #[test]
    fn test_parse_menu_callback_data() {
        assert_eq!(
            parse_menu_callback_data("m:home"),
            Some(MenuRequestAction::Page(MenuPage::Home))
        );
        assert_eq!(
            parse_menu_callback_data("m:new"),
            Some(MenuRequestAction::NewTransfer)
        );
        assert_eq!(
            parse_menu_callback_data("m:qtd"),
            Some(MenuRequestAction::QuickTransferDefault)
        );
        assert_eq!(
            parse_menu_callback_data("m:qlk"),
            Some(MenuRequestAction::NewLookup)
        );
        assert_eq!(
            parse_menu_callback_data("m:qld"),
            Some(MenuRequestAction::QuickLookupDefault)
        );
        assert_eq!(parse_menu_callback_data("x:new"), None);
    }

    // 首页应提供六个主要入口，保持日常操作足够短。
    #[test]
    fn test_home_buttons() {
        let rows = build_menu_buttons(MenuPage::Home, &[]);

        assert_eq!(rows[0][0].text, "转存");
        assert_eq!(rows[0][1].text, "下载");
        assert_eq!(rows[1][0].text, "查询");
    }

    // 下载按钮应直接复用 downloads callback，不让菜单重复实现分页逻辑。
    #[test]
    fn test_downloads_buttons_use_downloads_callback() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_menu_buttons(MenuPage::Downloads, &[]);

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

        let rows = build_menu_buttons(MenuPage::Help, &[]);

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("help button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert!(decoded.starts_with("h:"));
    }
}
