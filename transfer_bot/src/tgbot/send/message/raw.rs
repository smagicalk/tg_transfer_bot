// TDLib 消息请求封装。
// 这里保留最接近 TDLib JSON 协议的代码，上层只传已构造好的 `FormattedText`。

use serde_json::json;
use std::time::Duration;

use super::content::{build_text_input_message_content, parse_markdown_text};
use super::state::{wait_for_sent_message, wait_for_sent_message_id};
use crate::tgbot::TdError;

/// 发送文本消息并返回 TDLib 回传的消息对象。
///
/// 进度面板需要拿到 `message_id`，后续才能用 `editMessageText` 原地刷新。
pub(in crate::tgbot::send::message) async fn send_formatted_text_message_returning(
    text: tdlib_rs::types::FormattedText,
    chat_id: i64,
    reply_markup: Option<tdlib_rs::enums::ReplyMarkup>,
    client_id: i32,
) -> anyhow::Result<tdlib_rs::types::Message> {
    let response = tdlib_rs::send_request(
        client_id,
        json!({
            "@type": "sendMessage",
            "chat_id": chat_id,
            "topic_id": serde_json::Value::Null,
            "reply_to": serde_json::Value::Null,
            "options": serde_json::Value::Null,
            "reply_markup": reply_markup,
            "input_message_content": build_text_input_message_content(text),
        }),
    )
    .await;
    if response["@type"] == "error" {
        let err: tdlib_rs::types::Error = serde_json::from_value(response)?;
        return Err(anyhow::Error::new(TdError(err)));
    }
    let tdlib_rs::enums::Message::Message(message) = serde_json::from_value(response)?;
    wait_for_sent_message(message).await
}

/// 发送文本消息，可选附带 inline keyboard。
pub(in crate::tgbot::send::message) async fn send_formatted_text_message(
    text: tdlib_rs::types::FormattedText,
    chat_id: i64,
    reply_markup: Option<tdlib_rs::enums::ReplyMarkup>,
    client_id: i32,
) -> anyhow::Result<()> {
    let _ = send_formatted_text_message_returning(text, chat_id, reply_markup, client_id).await?;
    Ok(())
}

/// 编辑一条文本消息，并同步刷新 inline keyboard。
pub async fn edit_markdown_message_with_inline_keyboard(
    text: String,
    chat_id: i64,
    message_id: i64,
    keyboard: tdlib_rs::types::ReplyMarkupInlineKeyboard,
    client_id: i32,
) -> anyhow::Result<()> {
    let formatted_text = parse_markdown_text(text, client_id).await?;
    let response = send_edit_message_text(
        formatted_text.clone(),
        chat_id,
        message_id,
        keyboard.clone(),
        client_id,
    )
    .await?;
    if response["@type"] != "error" {
        return Ok(());
    }

    let err: tdlib_rs::types::Error = serde_json::from_value(response)?;
    if is_message_not_found(&err)
        && let Some(final_message_id) =
            wait_for_sent_message_id(chat_id, message_id, Duration::from_secs(30)).await
        && final_message_id != message_id
    {
        tracing::info!(
            chat_id,
            temporary_message_id = message_id,
            final_message_id,
            "retry edit message with final sent message id"
        );
        let retry_response = send_edit_message_text(
            formatted_text,
            chat_id,
            final_message_id,
            keyboard,
            client_id,
        )
        .await?;
        if retry_response["@type"] != "error" {
            return Ok(());
        }
        let retry_err: tdlib_rs::types::Error = serde_json::from_value(retry_response)?;
        return Err(anyhow::Error::new(TdError(retry_err)));
    }

    Err(anyhow::Error::new(TdError(err)))
}

/// 发送 editMessageText 原始请求，调用方负责解释 TDLib response。
async fn send_edit_message_text(
    formatted_text: tdlib_rs::types::FormattedText,
    chat_id: i64,
    message_id: i64,
    keyboard: tdlib_rs::types::ReplyMarkupInlineKeyboard,
    client_id: i32,
) -> anyhow::Result<serde_json::Value> {
    Ok(tdlib_rs::send_request(
        client_id,
        json!({
            "@type": "editMessageText",
            "chat_id": chat_id,
            "message_id": message_id,
            "reply_markup": tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard),
            "input_message_content": build_text_input_message_content(formatted_text),
        }),
    )
    .await)
}

/// 判断 TDLib 是否因为消息 ID 仍是临时 ID 而找不到消息。
fn is_message_not_found(err: &tdlib_rs::types::Error) -> bool {
    err.code == 400 && err.message.contains("Message not found")
}

/// 应答按钮回调，避免 Telegram 客户端一直转圈。
pub async fn answer_callback_query(
    callback_query_id: i64,
    text: Option<&str>,
    client_id: i32,
) -> anyhow::Result<()> {
    let response = tdlib_rs::send_request(
        client_id,
        json!({
            "@type": "answerCallbackQuery",
            "callback_query_id": callback_query_id,
            "text": text.unwrap_or(""),
            "show_alert": false,
            "url": "",
            "cache_time": 0,
        }),
    )
    .await;
    if response["@type"] == "error" {
        let err: tdlib_rs::types::Error = serde_json::from_value(response)?;
        return Err(anyhow::Error::new(TdError(err)));
    }
    Ok(())
}
