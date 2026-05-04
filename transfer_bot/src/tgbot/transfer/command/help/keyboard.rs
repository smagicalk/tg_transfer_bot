// `/help` 的复制按钮。
// Telegram 没有命令补全按钮，这里用 copy-text 降低手动输入成本。

use super::super::common::{
    CommandStyle, config_set_command, config_show_command, downloads_command,
    help_command as help_command_text, job_command,
};
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
pub(super) fn build_help_index_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_copy_button("复制 /t", "/t ", tdlib_rs::enums::ButtonStyle::Primary),
            send::build_copy_button("复制 /lk", "/lk ", tdlib_rs::enums::ButtonStyle::Default),
            send::build_copy_button("复制 /d", "/d", tdlib_rs::enums::ButtonStyle::Default),
            send::build_copy_button(
                "复制 /cfg",
                &config_show_command(CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
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
            help_nav_button("任务控制", "job", tdlib_rs::enums::ButtonStyle::Default),
            help_nav_button("运行配置", "config", tdlib_rs::enums::ButtonStyle::Default),
            help_nav_button("帮助说明", "help", tdlib_rs::enums::ButtonStyle::Default),
        ],
    ]
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
        ]],
        "transfer" => vec![vec![
            send::build_copy_button(
                "复制示例",
                "/t https://t.me/c/123/456",
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            help_index_button(),
        ]],
        "lookup" => vec![vec![
            send::build_copy_button(
                "复制示例",
                "/lk https://t.me/c/123/456",
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            help_index_button(),
        ]],
        "config" => vec![vec![
            send::build_copy_button(
                "复制 /cfg show",
                &config_show_command(CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制并发命令",
                &config_set_command("job_concurrency", 4, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            help_index_button(),
        ]],
        "downloads" => vec![vec![
            send::build_copy_button(
                "复制 /d",
                &downloads_command(None, None, None, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制 /d run",
                &downloads_command(Some("run"), None, None, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            help_index_button(),
        ]],
        "job" => vec![vec![
            send::build_copy_button(
                "复制暂停示例",
                &job_command("p", 123, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制恢复示例",
                &job_command("r", 123, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制停止示例",
                &job_command("s", 123, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制详情示例",
                &job_command("st", 123, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            help_index_button(),
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
