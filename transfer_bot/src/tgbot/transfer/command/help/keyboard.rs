// `/help` 的复制按钮。
// Telegram 没有命令补全按钮，这里用 copy-text 降低手动输入成本。

use super::super::common::{
    CommandStyle, balance_command, balance_history_command, cache_command, config_set_command,
    config_show_command, downloads_command, health_command, help_command as help_command_text,
    job_command, menu_command, points_change_command, points_history_command, points_show_command,
};
use super::super::menu::build_menu_home_callback_data;
use super::topic::normalize_help_topic;
use crate::tgbot::send;

/// `/help` 按钮回调前缀。
const HELP_CALLBACK_PREFIX: &str = "h:";

/// 判断 callback payload 是否属于 `/help`。
pub(super) fn is_help_callback_data(data: &str) -> bool {
    data.starts_with(HELP_CALLBACK_PREFIX)
}

/// 生成 help 页面切换按钮的 callback payload。
pub(super) fn build_help_callback_data(topic: Option<&str>) -> String {
    format!("{}{}", HELP_CALLBACK_PREFIX, topic.unwrap_or("index"))
}

/// 解析 help callback payload。
pub(super) fn parse_help_callback_data(data: &str) -> Option<Option<&str>> {
    let topic = data.strip_prefix(HELP_CALLBACK_PREFIX)?;
    match topic {
        "" | "index" => Some(None),
        other => normalize_help_topic(other).ok().map(Some),
    }
}

/// help 目录页按钮。
pub(super) fn build_help_index_buttons(
    is_admin: bool,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    // 目录页只保留最常用短命令，长命令和完整示例放到具体帮助详情页。
    // 这样首页不会变成“命令墙”，用户先点分类再展开即可。
    let mut rows = vec![
        vec![
            send::build_copy_button("复制 /t", "/t ", tdlib_rs::enums::ButtonStyle::Primary),
            send::build_copy_button("复制 /lk", "/lk ", tdlib_rs::enums::ButtonStyle::Default),
        ],
        vec![
            send::build_copy_button(
                "复制 /d",
                &downloads_command(None, None, None, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制 /bal",
                &balance_command(CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![send::build_copy_button(
            "复制流水",
            &balance_history_command(10, 1, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        )],
        vec![
            help_nav_button("转存", "transfer", tdlib_rs::enums::ButtonStyle::Primary),
            help_nav_button("查询", "lookup", tdlib_rs::enums::ButtonStyle::Default),
            help_nav_button(
                "下载列表",
                "downloads",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            help_nav_button("积分账户", "points", tdlib_rs::enums::ButtonStyle::Default),
            help_nav_button("任务控制", "job", tdlib_rs::enums::ButtonStyle::Default),
            help_nav_button("交互菜单", "menu", tdlib_rs::enums::ButtonStyle::Default),
        ],
        vec![help_nav_button(
            "帮助说明",
            "help",
            tdlib_rs::enums::ButtonStyle::Default,
        )],
        vec![menu_home_button()],
    ];

    if is_admin {
        rows.insert(
            2,
            vec![
                send::build_copy_button(
                    "复制 /hl",
                    &health_command(CommandStyle::Short),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
                send::build_copy_button(
                    "复制 /cfg",
                    &config_show_command(CommandStyle::Short),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
                send::build_copy_button(
                    "复制 /fc",
                    &cache_command(None, None, None, CommandStyle::Short),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
            ],
        );
        rows.insert(
            5,
            vec![
                help_nav_button("运行健康", "health", tdlib_rs::enums::ButtonStyle::Default),
                help_nav_button("文件缓存", "cache", tdlib_rs::enums::ButtonStyle::Default),
                help_nav_button("运行配置", "config", tdlib_rs::enums::ButtonStyle::Default),
            ],
        );
    }

    rows
}

/// 详细帮助页按钮。
pub(super) fn build_help_detail_buttons(
    command_name: &str,
) -> anyhow::Result<Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>> {
    let command_name = normalize_help_topic(command_name)?;
    let rows = match command_name {
        "help" => vec![vec![
            send::build_copy_button(
                "复制 /help",
                &help_command_text(None, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            help_index_button(),
            menu_home_button(),
        ]],
        "transfer" => vec![vec![
            send::build_copy_button(
                "复制示例",
                "/transfer https://t.me/c/123/456",
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            help_index_button(),
            menu_home_button(),
        ]],
        "lookup" => vec![vec![
            send::build_copy_button(
                "复制示例",
                "/lookup https://t.me/c/123/456",
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            help_index_button(),
            menu_home_button(),
        ]],
        "points" => vec![
            vec![
                send::build_copy_button(
                    "复制 /balance",
                    &balance_command(CommandStyle::Long),
                    tdlib_rs::enums::ButtonStyle::Primary,
                ),
                send::build_copy_button(
                    "复制查看",
                    &points_show_command(123456789, CommandStyle::Long),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
            ],
            vec![
                send::build_copy_button(
                    "复制流水",
                    &balance_history_command(10, 1, CommandStyle::Long),
                    tdlib_rs::enums::ButtonStyle::Primary,
                ),
                send::build_copy_button(
                    "复制用户流水",
                    &points_history_command(123456789, 10, 1, CommandStyle::Long),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
            ],
            vec![
                send::build_copy_button(
                    "复制加分",
                    &points_change_command(
                        "add",
                        123456789,
                        10,
                        "admin_adjust",
                        CommandStyle::Long,
                    ),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
                send::build_copy_button(
                    "复制扣分",
                    &points_change_command(
                        "sub",
                        123456789,
                        10,
                        "admin_adjust",
                        CommandStyle::Long,
                    ),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
            ],
            vec![help_index_button(), menu_home_button()],
        ],
        "health" => vec![vec![
            send::build_copy_button(
                "复制 /health",
                &health_command(CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            help_index_button(),
            menu_home_button(),
        ]],
        "cache" => vec![vec![
            send::build_copy_button(
                "复制 /cache",
                &cache_command(None, None, None, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制分页",
                &cache_command(Some("page"), None, None, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            help_index_button(),
            menu_home_button(),
        ]],
        "config" => vec![vec![
            send::build_copy_button(
                "复制 /config show",
                &config_show_command(CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制并发命令",
                &config_set_command("job_concurrency", 4, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            help_index_button(),
            menu_home_button(),
        ]],
        "downloads" => vec![vec![
            send::build_copy_button(
                "复制 /downloads",
                &downloads_command(None, None, None, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制 /downloads run",
                &downloads_command(Some("run"), None, None, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            help_index_button(),
            menu_home_button(),
        ]],
        "job" => vec![
            vec![
                send::build_copy_button(
                    "复制暂停示例",
                    &job_command("p", 123, CommandStyle::Long),
                    tdlib_rs::enums::ButtonStyle::Primary,
                ),
                send::build_copy_button(
                    "复制恢复示例",
                    &job_command("r", 123, CommandStyle::Long),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
            ],
            vec![
                send::build_copy_button(
                    "复制停止示例",
                    &job_command("s", 123, CommandStyle::Long),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
                send::build_copy_button(
                    "复制详情示例",
                    &job_command("st", 123, CommandStyle::Long),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
            ],
            vec![help_index_button(), menu_home_button()],
        ],
        "menu" => vec![vec![
            send::build_copy_button(
                "复制 /menu",
                &menu_command(CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            help_index_button(),
            menu_home_button(),
        ]],
        _ => anyhow::bail!("unknown help topic: {}", command_name),
    };
    Ok(rows)
}

/// 构建 help 页面切换按钮。
fn help_nav_button(
    text: &str,
    topic: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(text, &build_help_callback_data(Some(topic)), style)
}

/// 构建返回 help 目录按钮。
fn help_index_button() -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        "返回目录",
        &build_help_callback_data(None),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}

/// 构建返回主菜单按钮。
fn menu_home_button() -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        "菜单",
        &build_menu_home_callback_data(),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}
