// `/help` 的复制按钮。
// Telegram 没有命令补全按钮，这里用 copy-text 降低手动输入成本。

use super::super::common::{
    CommandStyle, config_set_command, config_show_command, downloads_command,
    help_command as help_command_text, job_command,
};
use super::topic::normalize_help_topic;
use crate::tgbot::send;

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
        vec![send::build_copy_button(
            "复制 /j",
            "/j ",
            tdlib_rs::enums::ButtonStyle::Default,
        )],
        vec![
            send::build_copy_button(
                "帮助 transfer",
                "/h transfer",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "帮助 downloads",
                "/h downloads",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button("帮助 job", "/h job", tdlib_rs::enums::ButtonStyle::Default),
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
            send::build_copy_button("返回目录", "/h", tdlib_rs::enums::ButtonStyle::Default),
        ]],
        "transfer" => vec![vec![
            send::build_copy_button(
                "复制示例",
                "/t https://t.me/c/123/456",
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button("返回目录", "/h", tdlib_rs::enums::ButtonStyle::Default),
        ]],
        "lookup" => vec![vec![
            send::build_copy_button(
                "复制示例",
                "/lk https://t.me/c/123/456",
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button("返回目录", "/h", tdlib_rs::enums::ButtonStyle::Default),
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
        ]],
        _ => anyhow::bail!("unknown help topic: {}", command_name),
    };
    Ok(rows)
}
