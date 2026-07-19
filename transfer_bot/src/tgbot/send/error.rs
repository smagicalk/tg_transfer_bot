// 统一交互错误卡片。
// callback 已经 ACK 后不能再次弹提示，因此这里把错误说明和“复制错误”按钮统一封装。

use super::{ReplyPanel, build_copy_button, edit_card_message_with_inline_keyboard};
use crate::tgbot::transfer::card;

/// 编辑交互卡片；如果原消息编辑失败，则发送独立错误卡片。
///
/// callback 已经 ACK 后，Telegram 客户端不会再展示第二次 callback 提示。
/// 因此编辑失败时必须发一条新消息，否则用户只会看到按钮转完圈但页面没有变化。
pub async fn edit_interaction_card_or_error(
    text: String,
    chat_id: i64,
    message_id: i64,
    keyboard: tdlib_rs::types::ReplyMarkupInlineKeyboard,
    client_id: i32,
    title: &str,
    detail: &str,
) -> anyhow::Result<()> {
    if let Err(err) =
        edit_card_message_with_inline_keyboard(text, chat_id, message_id, keyboard, client_id).await
    {
        send_interaction_error_card(chat_id, client_id, title, detail, &err).await?;
        return Err(err);
    }
    Ok(())
}

/// 发送统一的交互错误卡片。
///
/// 调用方只需要提供标题、简短说明和原始错误；这里负责统一排版和日志记录。
pub async fn send_interaction_error_card(
    request_chat_id: i64,
    client_id: i32,
    title: &str,
    detail: &str,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    let error_text = err.to_string();
    let result = ReplyPanel::card(interaction_error_text(title, detail, &error_text))
        .row(copy_error_row(&error_text))
        .send(request_chat_id, client_id)
        .await;
    if let Err(send_err) = &result {
        tracing::warn!(
            request_chat_id,
            client_id,
            title,
            error = %send_err,
            "send interaction error card failed"
        );
    }
    result
}

/// 构造统一的交互错误卡片正文。
fn interaction_error_text(title: &str, detail: &str, error_text: &str) -> String {
    [
        title.to_owned(),
        format!("状态：{}", card::code("failed")),
        card::DIVIDER.to_owned(),
        card::section("原因"),
        card::note(detail),
        card::section("错误"),
        card::pre_code(error_text),
    ]
    .join("\n")
}

/// 构造统一的“复制错误”按钮。
fn copy_error_row(error_text: &str) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![build_copy_button(
        "复制错误",
        error_text,
        tdlib_rs::enums::ButtonStyle::Default,
    )]
}

#[cfg(test)]
mod tests {
    use super::{copy_error_row, interaction_error_text};

    // 交互错误卡片应稳定保留状态、原因和错误块。
    #[test]
    fn test_interaction_error_text_layout() {
        let text = interaction_error_text("帮助刷新失败", "帮助页未更新。", "db timeout");

        assert!(text.contains("帮助刷新失败"));
        assert!(text.contains("状态：‹failed›"));
        assert!(text.contains("■ 原因"));
        assert!(text.contains("■ 错误"));
    }

    // 复制错误按钮应始终只生成一行，避免错误卡片布局漂移。
    #[test]
    fn test_copy_error_row_layout() {
        let row = copy_error_row("db timeout");

        assert_eq!(row.len(), 1);
        assert_eq!(row[0].text, "复制错误");
    }
}
