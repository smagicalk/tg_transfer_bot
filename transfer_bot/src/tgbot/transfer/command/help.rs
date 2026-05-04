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

/// 判断 callback payload 是否属于 `/help`。
pub(super) fn is_help_callback_data(data: &str) -> bool {
    keyboard::is_help_callback_data(data)
}

/// `/help` 命令入口。
/// 默认返回命令目录；带命令名时返回该命令的详细帮助。
pub async fn help_command(
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (help_text, rows) = build_help_page(text.get(1).copied())?;
    let mut panel = send::ReplyPanel::card(help_text);
    for row in rows {
        panel = panel.row(row);
    }
    panel.send(request_chat_id, client_id).await
}

/// `/help` inline keyboard 回调入口。
///
/// help 页只做“原地切换文案”，不会修改任务状态，所以适合使用 callback。
pub async fn help_callback_query(
    update: tdlib_rs::enums::UpdateNewCallbackQuery,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };

    let Some(topic) = keyboard::parse_help_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("帮助按钮参数无效"), client_id).await?;
        return Ok(());
    };

    let (text, rows) = build_help_page(topic)?;
    let (text, keyboard) = send::ReplyPanel::card(text).rows(rows).into_card_parts()?;
    send::answer_callback_query(update.id, Some("已切换帮助"), client_id).await?;
    send::edit_card_message_with_inline_keyboard(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
    )
    .await
}

/// 构建 help 目录页或详情页，供文本命令和 callback 共用。
fn build_help_page(
    command_name: Option<&str>,
) -> anyhow::Result<(String, Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>)> {
    match command_name {
        None => Ok((build_help_index_text(), build_help_index_buttons())),
        Some(command_name) => Ok((
            build_help_detail_text(command_name)?,
            build_help_detail_buttons(command_name)?,
        )),
    }
}
