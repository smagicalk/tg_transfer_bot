// `/help` 命令入口。
// 具体文案、按钮和 topic 归一化分别放在子模块，避免帮助文案撑大入口文件。

use crate::tgbot::send;

mod keyboard;
mod text;
mod topic;

#[cfg(test)]
mod tests;

use keyboard::{build_help_detail_buttons, build_help_index_buttons};
use text::{build_help_detail_text, build_help_index_text};

/// `/help` 命令入口。
/// 默认返回命令目录；带命令名时返回该命令的详细帮助。
pub async fn help_command(
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (help_text, rows) = match text.get(1).copied() {
        None => (build_help_index_text(), build_help_index_buttons()),
        Some(command_name) => (
            build_help_detail_text(command_name)?,
            build_help_detail_buttons(command_name)?,
        ),
    };
    let mut panel = send::ReplyPanel::card(help_text);
    for row in rows {
        panel = panel.row(row);
    }
    panel.send(request_chat_id, client_id).await
}
