// TDLib 消息请求封装。
// 这里保留最接近 TDLib JSON 协议的代码，上层只传已构造好的 `FormattedText`。

use serde_json::json;
use std::time::Duration;

use super::content::{build_card_formatted_text, parse_markdown_text};
use super::is_reply_markup_enabled;
use super::state::{wait_for_sent_message, wait_for_sent_message_id};
use crate::tgbot::TdError;

/// 发送消息后业务层真正需要的最小消息字段。
///
/// TDLib 成功响应里会带回完整 `message`，其中可能包含 last_message/reply_markup 等复杂嵌套。
/// 对机器人回复来说只需要消息 ID 和发送状态，所以先解析成轻量结构，降低 worker 栈压力。
#[derive(Debug, serde::Deserialize)]
#[cfg(test)]
struct SentMessageLite {
    id: i64,
    chat_id: i64,
    #[serde(default)]
    sending_state: Option<serde_json::Value>,
}

/// 发送文本消息并返回 TDLib 回传的消息对象。
///
/// 进度面板需要拿到 `message_id`，后续才能用 `editMessageText` 原地刷新。
pub(in crate::tgbot::send::message) async fn send_formatted_text_message_returning(
    text: tdlib_rs::types::FormattedText,
    chat_id: i64,
    reply_markup: Option<tdlib_rs::enums::ReplyMarkup>,
    client_id: i32,
) -> anyhow::Result<tdlib_rs::types::Message> {
    let reply_markup_enabled = is_reply_markup_enabled();
    let (text, reply_markup) = apply_reply_markup_capability(
        text,
        reply_markup,
        reply_markup_enabled,
        chat_id,
        None,
        "sendMessage",
    );
    tracing::debug!(
        chat_id,
        has_reply_markup = reply_markup.is_some(),
        reply_markup_enabled,
        text_len = text.text.chars().count(),
        "tdlib sendMessage requested"
    );
    // 生成函数会负责真正调用 TDLib；这里仍先复用本模块的校验，避免把不支持的按钮类型发出去。
    let _ = prepare_optional_reply_markup(reply_markup.clone())?;
    let sent = match tdlib_rs::functions::send_message(
        chat_id,
        None,
        None,
        None,
        reply_markup,
        build_input_message_text_content(text),
        client_id,
    )
    .await
    {
        Ok(sent) => sent,
        Err(err) => {
            tracing::warn!(
                chat_id,
                error_code = err.code,
                error_message = %err.message,
                "tdlib sendMessage returned error"
            );
            return Err(anyhow::Error::new(TdError(err)));
        }
    };
    let tdlib_rs::enums::Message::Message(message) = sent;
    tracing::debug!(
        chat_id = message.chat_id,
        message_id = message.id,
        is_temporary = message.sending_state.is_some(),
        "tdlib sendMessage initial response received"
    );
    wait_for_sent_message(message, client_id).await
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
    edit_formatted_message_with_inline_keyboard(
        formatted_text,
        chat_id,
        message_id,
        keyboard,
        client_id,
    )
    .await
}

/// 编辑一条卡片风格文本消息，并同步刷新 inline keyboard。
///
/// 卡片文本在本地转换成 TDLib `FormattedText`，用于进度面板这类需要频繁编辑的回复。
pub async fn edit_card_message_with_inline_keyboard(
    text: String,
    chat_id: i64,
    message_id: i64,
    keyboard: tdlib_rs::types::ReplyMarkupInlineKeyboard,
    client_id: i32,
) -> anyhow::Result<()> {
    let formatted_text = build_card_formatted_text(text)?;
    edit_formatted_message_with_inline_keyboard(
        formatted_text,
        chat_id,
        message_id,
        keyboard,
        client_id,
    )
    .await
}

/// 编辑一条已经构造好的 `FormattedText` 消息。
async fn edit_formatted_message_with_inline_keyboard(
    formatted_text: tdlib_rs::types::FormattedText,
    chat_id: i64,
    message_id: i64,
    keyboard: tdlib_rs::types::ReplyMarkupInlineKeyboard,
    client_id: i32,
) -> anyhow::Result<()> {
    let reply_markup_enabled = is_reply_markup_enabled();
    let (formatted_text, reply_markup) = apply_reply_markup_capability(
        formatted_text,
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard)),
        reply_markup_enabled,
        chat_id,
        Some(message_id),
        "editMessageText",
    );
    let keyboard = inline_keyboard_from_reply_markup(reply_markup);
    let edit_result = send_edit_message_text(
        formatted_text.clone(),
        chat_id,
        message_id,
        keyboard.clone(),
        client_id,
    )
    .await?;
    let err = match edit_result {
        Ok(()) => return Ok(()),
        Err(err) => err,
    };
    if is_message_not_modified(&err) {
        // TDLib 对“文本和按钮都没变化”的编辑会返回 400。
        // 对刷新面板来说这是幂等成功，不应污染日志或中断 callback。
        tracing::debug!(
            chat_id,
            message_id,
            "skip edit message because content is unchanged"
        );
        return Ok(());
    }

    if is_message_not_found(&err)
        && let Some(final_message_id) =
            wait_for_sent_message_id(client_id, chat_id, message_id, Duration::from_secs(30)).await
        && final_message_id != message_id
    {
        tracing::info!(
            chat_id,
            temporary_message_id = message_id,
            final_message_id,
            "retry edit message with final sent message id"
        );
        let retry_result = send_edit_message_text(
            formatted_text,
            chat_id,
            final_message_id,
            keyboard,
            client_id,
        )
        .await?;
        let retry_err = match retry_result {
            Ok(()) => return Ok(()),
            Err(err) => err,
        };
        if is_message_not_modified(&retry_err) {
            tracing::debug!(
                chat_id,
                final_message_id,
                "skip retry edit message because content is unchanged"
            );
            return Ok(());
        }
        return Err(anyhow::Error::new(TdError(retry_err)));
    }

    Err(anyhow::Error::new(TdError(err)))
}

/// 发送 editMessageText 原始请求，调用方负责解释 TDLib response。
async fn send_edit_message_text(
    formatted_text: tdlib_rs::types::FormattedText,
    chat_id: i64,
    message_id: i64,
    keyboard: Option<tdlib_rs::types::ReplyMarkupInlineKeyboard>,
    client_id: i32,
) -> anyhow::Result<Result<(), tdlib_rs::types::Error>> {
    let reply_markup = keyboard.map(tdlib_rs::enums::ReplyMarkup::InlineKeyboard);
    // 先走显式校验，保持“只支持本项目允许的按钮类型”的边界。
    let _ = prepare_optional_reply_markup(reply_markup.clone())?;
    Ok(tdlib_rs::functions::edit_message_text(
        chat_id,
        message_id,
        reply_markup,
        build_input_message_text_content(formatted_text),
        client_id,
    )
    .await
    .map(|_| ()))
}

/// 构造 `sendMessage` 请求 JSON。
///
/// 发送层显式拼 JSON，而不是把生成的 TDLib enum 直接塞进 `json!`。
/// 这样可以把 bytes/base64、可支持按钮类型和 null 字段都集中在一个可测试边界。
#[cfg(test)]
fn build_send_message_request(
    text: tdlib_rs::types::FormattedText,
    chat_id: i64,
    reply_markup: serde_json::Value,
) -> serde_json::Value {
    json!({
        "@type": "sendMessage",
        "chat_id": chat_id,
        "topic_id": serde_json::Value::Null,
        "reply_to": serde_json::Value::Null,
        "options": serde_json::Value::Null,
        "reply_markup": reply_markup,
        "input_message_content": build_input_message_text_value(text),
    })
}

/// 构造 `editMessageText` 请求 JSON。
#[cfg(test)]
fn build_edit_message_text_request(
    text: tdlib_rs::types::FormattedText,
    chat_id: i64,
    message_id: i64,
    reply_markup: serde_json::Value,
) -> serde_json::Value {
    json!({
        "@type": "editMessageText",
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
        "input_message_content": build_input_message_text_value(text),
    })
}

/// 将可选 reply_markup 转成 TDLib JSON。
fn prepare_optional_reply_markup(
    reply_markup: Option<tdlib_rs::enums::ReplyMarkup>,
) -> anyhow::Result<serde_json::Value> {
    reply_markup
        .map(prepare_reply_markup)
        .transpose()
        .map(|value| value.unwrap_or(serde_json::Value::Null))
}

/// 按当前账号能力过滤 reply_markup。
///
/// 用户号登录时统一丢弃 reply_markup，而不是让每个命令模块分别判断按钮是否可用。
/// 如果按钮里有 copy/url 内容，会追加到正文，避免隐藏按钮后丢失命令和结果链接。
fn apply_reply_markup_capability(
    text: tdlib_rs::types::FormattedText,
    reply_markup: Option<tdlib_rs::enums::ReplyMarkup>,
    reply_markup_enabled: bool,
    chat_id: i64,
    message_id: Option<i64>,
    operation: &'static str,
) -> (
    tdlib_rs::types::FormattedText,
    Option<tdlib_rs::enums::ReplyMarkup>,
) {
    if reply_markup_enabled || reply_markup.is_none() {
        return (text, reply_markup);
    }

    let text = if let Some(reply_markup) = reply_markup.as_ref() {
        append_reply_markup_fallback_text(text, reply_markup)
    } else {
        text
    };
    let reply_markup = filter_reply_markup_by_capability(
        reply_markup,
        reply_markup_enabled,
        chat_id,
        message_id,
        operation,
    );
    (text, reply_markup)
}

/// 按当前账号能力过滤 reply_markup。
///
/// 用户号登录时统一丢弃 reply_markup，而不是让每个命令模块分别判断按钮是否可用。
fn filter_reply_markup_by_capability(
    reply_markup: Option<tdlib_rs::enums::ReplyMarkup>,
    reply_markup_enabled: bool,
    chat_id: i64,
    message_id: Option<i64>,
    operation: &'static str,
) -> Option<tdlib_rs::enums::ReplyMarkup> {
    if reply_markup_enabled || reply_markup.is_none() {
        return reply_markup;
    }

    tracing::debug!(
        chat_id,
        message_id,
        operation,
        "drop reply markup because current login mode cannot show buttons"
    );
    None
}

/// 从 reply_markup 里取出 inline keyboard。
fn inline_keyboard_from_reply_markup(
    reply_markup: Option<tdlib_rs::enums::ReplyMarkup>,
) -> Option<tdlib_rs::types::ReplyMarkupInlineKeyboard> {
    let reply_markup = reply_markup?;
    let tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard) = reply_markup else {
        return None;
    };
    Some(keyboard)
}

/// 将按钮里的可复制内容追加到正文，作为用户号模式的文本降级。
fn append_reply_markup_fallback_text(
    mut text: tdlib_rs::types::FormattedText,
    reply_markup: &tdlib_rs::enums::ReplyMarkup,
) -> tdlib_rs::types::FormattedText {
    let fallback_lines = reply_markup_fallback_lines(reply_markup);
    if fallback_lines.is_empty() {
        return text;
    }

    text.text.push_str("\n\n可复制内容：");
    for line in fallback_lines {
        text.text.push('\n');
        text.text.push_str("- ");
        text.text.push_str(&line);
    }
    text
}

/// 提取 copy/url 按钮内容。
///
/// callback 按钮只在 bot 模式有意义，用户号模式没有可直接复制的 payload，不追加到正文。
fn reply_markup_fallback_lines(reply_markup: &tdlib_rs::enums::ReplyMarkup) -> Vec<String> {
    let tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard) = reply_markup else {
        return Vec::new();
    };

    let mut lines = Vec::new();
    for button in keyboard.rows.iter().flatten() {
        let Some(line) = inline_button_fallback_line(button) else {
            continue;
        };
        if !lines.contains(&line) {
            lines.push(line);
        }
        if lines.len() >= 12 {
            break;
        }
    }
    lines
}

/// 提取单个 inline button 的文本降级内容。
fn inline_button_fallback_line(button: &tdlib_rs::types::InlineKeyboardButton) -> Option<String> {
    match &button.r#type {
        tdlib_rs::enums::InlineKeyboardButtonType::CopyText(copy_text) => {
            Some(format!("{}：{}", button.text, copy_text.text))
        }
        tdlib_rs::enums::InlineKeyboardButtonType::Url(url) => {
            Some(format!("{}：{}", button.text, url.url))
        }
        _ => None,
    }
}

/// 将当前业务支持的 reply markup 显式转成 TDLib JSON。
fn prepare_reply_markup(
    reply_markup: tdlib_rs::enums::ReplyMarkup,
) -> anyhow::Result<serde_json::Value> {
    match reply_markup {
        tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard) => {
            build_inline_keyboard_value(keyboard)
        }
        tdlib_rs::enums::ReplyMarkup::ForceReply(force_reply) => Ok(json!({
            "@type": "replyMarkupForceReply",
            "is_personal": force_reply.is_personal,
            "input_field_placeholder": force_reply.input_field_placeholder,
        })),
        tdlib_rs::enums::ReplyMarkup::ShowKeyboard(keyboard) => build_show_keyboard_value(keyboard),
        tdlib_rs::enums::ReplyMarkup::RemoveKeyboard(remove_keyboard) => Ok(json!({
            "@type": "replyMarkupRemoveKeyboard",
            "is_personal": remove_keyboard.is_personal,
        })),
    }
}

/// 构造 reply keyboard JSON。
///
/// 当前只需要 Telegram 原生选聊按钮，其他 reply keyboard 类型先不开放，避免无意支持
/// 电话、位置等敏感输入。
fn build_show_keyboard_value(
    keyboard: tdlib_rs::types::ReplyMarkupShowKeyboard,
) -> anyhow::Result<serde_json::Value> {
    let rows = keyboard
        .rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(build_keyboard_button_value)
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(json!({
        "@type": "replyMarkupShowKeyboard",
        "rows": rows,
        "is_persistent": keyboard.is_persistent,
        "resize_keyboard": keyboard.resize_keyboard,
        "one_time": keyboard.one_time,
        "is_personal": keyboard.is_personal,
        "input_field_placeholder": keyboard.input_field_placeholder,
    }))
}

/// 构造单个 reply keyboard button JSON。
fn build_keyboard_button_value(
    button: tdlib_rs::types::KeyboardButton,
) -> anyhow::Result<serde_json::Value> {
    let button_type = match button.r#type {
        tdlib_rs::enums::KeyboardButtonType::Text => json!({
            "@type": "keyboardButtonTypeText",
        }),
        tdlib_rs::enums::KeyboardButtonType::RequestChat(request) => json!({
            "@type": "keyboardButtonTypeRequestChat",
            "id": request.id,
            "chat_is_channel": request.chat_is_channel,
            "restrict_chat_is_forum": request.restrict_chat_is_forum,
            "chat_is_forum": request.chat_is_forum,
            "restrict_chat_has_username": request.restrict_chat_has_username,
            "chat_has_username": request.chat_has_username,
            "chat_is_created": request.chat_is_created,
            "user_administrator_rights": request.user_administrator_rights,
            "bot_administrator_rights": request.bot_administrator_rights,
            "bot_is_member": request.bot_is_member,
            "request_title": request.request_title,
            "request_username": request.request_username,
            "request_photo": request.request_photo,
        }),
        _ => anyhow::bail!("unsupported reply keyboard button type"),
    };

    Ok(json!({
        "text": button.text,
        "icon_custom_emoji_id": button.icon_custom_emoji_id.to_string(),
        "style": build_button_style_value(button.style),
        "type": button_type,
    }))
}

/// 构造 inline keyboard JSON。
fn build_inline_keyboard_value(
    keyboard: tdlib_rs::types::ReplyMarkupInlineKeyboard,
) -> anyhow::Result<serde_json::Value> {
    let rows = keyboard
        .rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(build_inline_keyboard_button_value)
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(json!({
        "@type": "replyMarkupInlineKeyboard",
        "rows": rows,
    }))
}

/// 构造单个 inline keyboard button JSON。
fn build_inline_keyboard_button_value(
    button: tdlib_rs::types::InlineKeyboardButton,
) -> anyhow::Result<serde_json::Value> {
    let button_type = match button.r#type {
        tdlib_rs::enums::InlineKeyboardButtonType::Url(url) => json!({
            "@type": "inlineKeyboardButtonTypeUrl",
            "url": url.url,
        }),
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) => json!({
            "@type": "inlineKeyboardButtonTypeCallback",
            "data": callback.data,
        }),
        tdlib_rs::enums::InlineKeyboardButtonType::CopyText(copy_text) => json!({
            "@type": "inlineKeyboardButtonTypeCopyText",
            "text": copy_text.text,
        }),
        _ => anyhow::bail!("unsupported inline keyboard button type"),
    };

    Ok(json!({
        "text": button.text,
        "icon_custom_emoji_id": button.icon_custom_emoji_id.to_string(),
        "style": build_button_style_value(button.style),
        "type": button_type,
    }))
}

/// 构造按钮样式 JSON。
fn build_button_style_value(style: tdlib_rs::enums::ButtonStyle) -> serde_json::Value {
    let style_type = match style {
        tdlib_rs::enums::ButtonStyle::Default => "buttonStyleDefault",
        tdlib_rs::enums::ButtonStyle::Primary => "buttonStylePrimary",
        tdlib_rs::enums::ButtonStyle::Danger => "buttonStyleDanger",
        tdlib_rs::enums::ButtonStyle::Success => "buttonStyleSuccess",
    };
    json!({ "@type": style_type })
}

/// 构造文本消息内容 JSON。
#[cfg(test)]
fn build_input_message_text_value(text: tdlib_rs::types::FormattedText) -> serde_json::Value {
    json!({
        "@type": "inputMessageText",
        "text": text,
        "link_preview_options": serde_json::Value::Null,
        "clear_draft": true,
    })
}

/// 构造生成函数需要的文本消息内容。
fn build_input_message_text_content(
    text: tdlib_rs::types::FormattedText,
) -> tdlib_rs::enums::InputMessageContent {
    tdlib_rs::enums::InputMessageContent::InputMessageText(tdlib_rs::types::InputMessageText {
        text,
        link_preview_options: None,
        clear_draft: true,
    })
}

/// 轻量解析 `sendMessage` 返回的 message。
#[cfg(test)]
fn parse_sent_message_lite(
    response: serde_json::Value,
) -> anyhow::Result<tdlib_rs::types::Message> {
    let lite: SentMessageLite = serde_json::from_value(response)?;
    Ok(build_minimal_sent_message(lite))
}

/// 把轻量消息转成现有等待状态模块需要的 TDLib message。
#[cfg(test)]
fn build_minimal_sent_message(lite: SentMessageLite) -> tdlib_rs::types::Message {
    tdlib_rs::types::Message {
        id: lite.id,
        sender_id: tdlib_rs::enums::MessageSender::User(tdlib_rs::types::MessageSenderUser {
            user_id: 0,
        }),
        chat_id: lite.chat_id,
        sending_state: lite.sending_state.map(|_| {
            tdlib_rs::enums::MessageSendingState::Pending(
                tdlib_rs::types::MessageSendingStatePending { sending_id: 0 },
            )
        }),
        scheduling_state: None,
        is_outgoing: true,
        is_pinned: false,
        is_from_offline: false,
        can_be_saved: true,
        has_timestamped_media: false,
        is_channel_post: false,
        is_paid_star_suggested_post: false,
        is_paid_ton_suggested_post: false,
        contains_unread_mention: false,
        date: 0,
        edit_date: 0,
        forward_info: None,
        import_info: None,
        interaction_info: None,
        unread_reactions: vec![],
        fact_check: None,
        suggested_post_info: None,
        reply_to: None,
        topic_id: None,
        self_destruct_type: None,
        self_destruct_in: 0.0,
        auto_delete_in: 0.0,
        via_bot_user_id: 0,
        sender_business_bot_user_id: 0,
        sender_boost_count: 0,
        sender_tag: String::new(),
        paid_message_star_count: 0,
        author_signature: String::new(),
        media_album_id: 0,
        effect_id: 0,
        restriction_info: None,
        summary_language_code: String::new(),
        content: tdlib_rs::enums::MessageContent::MessageText(tdlib_rs::types::MessageText {
            text: tdlib_rs::types::FormattedText {
                text: String::new(),
                entities: vec![],
            },
            link_preview: None,
            link_preview_options: None,
        }),
        reply_markup: None,
    }
}

/// 判断 TDLib 是否因为消息 ID 仍是临时 ID 而找不到消息。
fn is_message_not_found(err: &tdlib_rs::types::Error) -> bool {
    err.code == 400 && err.message.contains("Message not found")
}

/// 判断 TDLib 是否因为消息内容和按钮没有变化而拒绝编辑。
fn is_message_not_modified(err: &tdlib_rs::types::Error) -> bool {
    err.code == 400
        && (err.message.contains("MESSAGE_NOT_MODIFIED")
            || err.message.to_ascii_lowercase().contains("not modified")
            || err
                .message
                .to_ascii_lowercase()
                .contains("message is not modified"))
}

/// 应答按钮回调，避免 Telegram 客户端一直转圈。
pub async fn answer_callback_query(
    callback_query_id: i64,
    text: Option<&str>,
    client_id: i32,
) -> anyhow::Result<()> {
    if let Err(err) = tdlib_rs::functions::answer_callback_query(
        callback_query_id,
        text.unwrap_or("").to_owned(),
        false,
        String::new(),
        0,
        client_id,
    )
    .await
    {
        tracing::warn!(
            callback_query_id,
            error_code = err.code,
            error_message = %err.message,
            "tdlib answerCallbackQuery returned error"
        );
        return Err(anyhow::Error::new(TdError(err)));
    }
    tracing::debug!(callback_query_id, "tdlib answerCallbackQuery completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        apply_reply_markup_capability, build_edit_message_text_request, build_send_message_request,
        filter_reply_markup_by_capability, inline_keyboard_from_reply_markup, is_message_not_found,
        is_message_not_modified, parse_sent_message_lite, prepare_optional_reply_markup,
    };
    use crate::tgbot::send::{
        build_callback_button, build_copy_button, build_inline_keyboard, build_url_button,
    };

    // “内容未变化”对刷新型面板是幂等成功，调用方不需要感知错误。
    #[test]
    fn test_is_message_not_modified() {
        let err = tdlib_rs::types::Error {
            code: 400,
            message: "MESSAGE_NOT_MODIFIED".to_owned(),
        };
        assert!(is_message_not_modified(&err));

        let text_err = tdlib_rs::types::Error {
            code: 400,
            message: "Message is not modified".to_owned(),
        };
        assert!(is_message_not_modified(&text_err));
    }

    // 临时 message_id 的兜底只应匹配明确的 Message not found。
    #[test]
    fn test_is_message_not_found() {
        let err = tdlib_rs::types::Error {
            code: 400,
            message: "Message not found".to_owned(),
        };
        assert!(is_message_not_found(&err));
        assert!(!is_message_not_modified(&err));
    }

    // 发送 `/help` 后 TDLib 会在响应和 update 中带回 inline keyboard。
    // 这里用最小 message JSON 验证 `tdlib_rs` 的完整 Message 解析不会因为按钮结构直接崩溃。
    #[test]
    fn test_help_like_message_response_can_deserialize() {
        let keyboard = build_inline_keyboard(vec![
            vec![build_copy_button(
                "复制 /transfer",
                "/transfer ",
                tdlib_rs::enums::ButtonStyle::Primary,
            )],
            vec![build_callback_button(
                "转存",
                "h:transfer",
                tdlib_rs::enums::ButtonStyle::Default,
            )],
        ]);
        let reply_markup =
            serde_json::to_value(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard)).unwrap();

        let mut response: serde_json::Value = serde_json::from_str(
            r#"{
                "@type": "message",
                "id": 100,
                "sender_id": {"@type": "messageSenderUser", "user_id": 1},
                "chat_id": 1,
                "sending_state": null,
                "scheduling_state": null,
                "is_outgoing": true,
                "is_pinned": false,
                "is_from_offline": false,
                "can_be_saved": true,
                "has_timestamped_media": false,
                "is_channel_post": false,
                "is_paid_star_suggested_post": false,
                "is_paid_ton_suggested_post": false,
                "contains_unread_mention": false,
                "date": 0,
                "edit_date": 0,
                "forward_info": null,
                "import_info": null,
                "interaction_info": null,
                "unread_reactions": [],
                "fact_check": null,
                "suggested_post_info": null,
                "reply_to": null,
                "topic_id": null,
                "self_destruct_type": null,
                "self_destruct_in": 0.0,
                "auto_delete_in": 0.0,
                "via_bot_user_id": 0,
                "sender_business_bot_user_id": 0,
                "sender_boost_count": 0,
                "sender_tag": "",
                "paid_message_star_count": 0,
                "author_signature": "",
                "media_album_id": "0",
                "effect_id": "0",
                "restriction_info": null,
                "summary_language_code": "",
                "content": {
                    "@type": "messageText",
                    "text": {"text": "命令中心", "entities": []},
                    "link_preview": null,
                    "link_preview_options": null
                },
                "reply_markup": null
            }"#,
        )
        .unwrap();
        response["reply_markup"] = reply_markup;

        let tdlib_rs::enums::Message::Message(message) =
            serde_json::from_value(response).expect("help-like message should deserialize");
        assert_eq!(message.id, 100);
        assert!(message.reply_markup.is_some());
    }

    // `/help` 这类带按钮卡片应能构造成 TDLib `sendMessage` JSON 请求。
    // 这能锁住请求构造层，避免后续再次出现 bytes 字段或 enum 序列化问题。
    #[test]
    fn test_help_like_send_message_request_can_serialize() {
        let formatted_text = tdlib_rs::types::FormattedText {
            text: "命令中心".to_owned(),
            entities: vec![],
        };
        let keyboard = build_inline_keyboard(vec![
            vec![build_copy_button(
                "复制 /transfer",
                "/transfer ",
                tdlib_rs::enums::ButtonStyle::Primary,
            )],
            vec![build_callback_button(
                "转存",
                "h:transfer",
                tdlib_rs::enums::ButtonStyle::Default,
            )],
        ]);

        let request = build_send_message_request(
            formatted_text,
            1,
            prepare_optional_reply_markup(Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(
                keyboard,
            )))
            .unwrap(),
        );

        assert_eq!(request["@type"], "sendMessage");
        assert_eq!(
            request["reply_markup"]["@type"],
            "replyMarkupInlineKeyboard"
        );
        assert_eq!(
            request["input_message_content"]["@type"],
            "inputMessageText"
        );
    }

    // 私聊 bot 的原生选聊按钮需要 replyMarkupShowKeyboard，而不是 inline keyboard。
    #[test]
    fn test_request_chat_keyboard_send_message_request_can_serialize() {
        let formatted_text = tdlib_rs::types::FormattedText {
            text: "请选择目标聊天".to_owned(),
            entities: vec![],
        };
        let keyboard = tdlib_rs::types::ReplyMarkupShowKeyboard {
            rows: vec![
                vec![tdlib_rs::types::KeyboardButton {
                    text: "选择聊天".to_owned(),
                    icon_custom_emoji_id: 0,
                    style: tdlib_rs::enums::ButtonStyle::Primary,
                    r#type: tdlib_rs::enums::KeyboardButtonType::RequestChat(
                        tdlib_rs::types::KeyboardButtonTypeRequestChat {
                            id: 7001,
                            chat_is_channel: false,
                            restrict_chat_is_forum: false,
                            chat_is_forum: false,
                            restrict_chat_has_username: false,
                            chat_has_username: false,
                            chat_is_created: false,
                            user_administrator_rights: None,
                            bot_administrator_rights: None,
                            bot_is_member: true,
                            request_title: false,
                            request_username: false,
                            request_photo: false,
                        },
                    ),
                }],
                vec![tdlib_rs::types::KeyboardButton {
                    text: "取消".to_owned(),
                    icon_custom_emoji_id: 0,
                    style: tdlib_rs::enums::ButtonStyle::Danger,
                    r#type: tdlib_rs::enums::KeyboardButtonType::Text,
                }],
            ],
            is_persistent: false,
            resize_keyboard: true,
            one_time: true,
            is_personal: true,
            input_field_placeholder: "选择目标聊天".to_owned(),
        };

        let request = build_send_message_request(
            formatted_text,
            1,
            prepare_optional_reply_markup(Some(tdlib_rs::enums::ReplyMarkup::ShowKeyboard(
                keyboard,
            )))
            .unwrap(),
        );

        assert_eq!(request["reply_markup"]["@type"], "replyMarkupShowKeyboard");
        assert_eq!(
            request["reply_markup"]["rows"][0][0]["type"]["@type"],
            "keyboardButtonTypeRequestChat"
        );
        assert_eq!(request["reply_markup"]["rows"][0][0]["type"]["id"], 7001);
        assert_eq!(
            request["reply_markup"]["rows"][0][0]["type"]["bot_is_member"],
            true
        );
        assert_eq!(
            request["reply_markup"]["rows"][1][0]["type"]["@type"],
            "keyboardButtonTypeText"
        );
    }

    // 选聊结束或取消后必须能发送 replyMarkupRemoveKeyboard，避免客户端残留旧的选聊键盘。
    #[test]
    fn test_remove_keyboard_send_message_request_can_serialize() {
        let formatted_text = tdlib_rs::types::FormattedText {
            text: "已选择目标".to_owned(),
            entities: vec![],
        };

        let request = build_send_message_request(
            formatted_text,
            1,
            prepare_optional_reply_markup(Some(tdlib_rs::enums::ReplyMarkup::RemoveKeyboard(
                tdlib_rs::types::ReplyMarkupRemoveKeyboard { is_personal: true },
            )))
            .unwrap(),
        );

        assert_eq!(
            request["reply_markup"]["@type"],
            "replyMarkupRemoveKeyboard"
        );
        assert_eq!(request["reply_markup"]["is_personal"], true);
    }

    // 用户号模式下应统一丢弃 reply_markup，避免 TDLib 请求看似带按钮但客户端实际不显示。
    #[test]
    fn test_filter_reply_markup_by_capability_drops_buttons_when_disabled() {
        let keyboard = build_inline_keyboard(vec![vec![build_callback_button(
            "刷新",
            "d:r:all:8:1",
            tdlib_rs::enums::ButtonStyle::Default,
        )]]);

        let filtered = filter_reply_markup_by_capability(
            Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(
                keyboard.clone(),
            )),
            false,
            1,
            None,
            "sendMessage",
        );
        assert!(filtered.is_none());

        let kept = filter_reply_markup_by_capability(
            Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard)),
            true,
            1,
            None,
            "sendMessage",
        );
        assert!(kept.is_some());
    }

    // 编辑进度面板时也必须遵守同一能力开关，用户号模式下不能再附带不可见按钮。
    #[test]
    fn test_filter_inline_keyboard_by_capability_drops_edit_keyboard_when_disabled() {
        let keyboard = build_inline_keyboard(vec![vec![build_callback_button(
            "刷新",
            "d:r:all:8:1",
            tdlib_rs::enums::ButtonStyle::Default,
        )]]);

        let (_text, filtered) = apply_reply_markup_capability(
            tdlib_rs::types::FormattedText {
                text: "进度".to_owned(),
                entities: vec![],
            },
            Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard)),
            false,
            1,
            Some(100),
            "editMessageText",
        );
        let filtered = inline_keyboard_from_reply_markup(filtered);
        let request = build_edit_message_text_request(
            tdlib_rs::types::FormattedText {
                text: "进度".to_owned(),
                entities: vec![],
            },
            1,
            100,
            prepare_optional_reply_markup(
                filtered.map(tdlib_rs::enums::ReplyMarkup::InlineKeyboard),
            )
            .unwrap(),
        );

        assert_eq!(request["reply_markup"], serde_json::Value::Null);
    }

    // 用户号模式下隐藏按钮时，应把 copy/url 按钮内容补进正文，避免命令和结果链接丢失。
    #[test]
    fn test_apply_reply_markup_capability_appends_copy_and_url_fallback_text() {
        let keyboard = build_inline_keyboard(vec![vec![
            build_copy_button(
                "复制查询命令",
                "/lookup https://t.me/c/1/2 -100",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            build_url_button(
                "打开转存消息",
                "https://t.me/c/3/4",
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            build_callback_button("刷新", "d:r:all:8:1", tdlib_rs::enums::ButtonStyle::Default),
        ]]);

        let (text, reply_markup) = apply_reply_markup_capability(
            tdlib_rs::types::FormattedText {
                text: "查询结果".to_owned(),
                entities: vec![],
            },
            Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard)),
            false,
            1,
            None,
            "sendMessage",
        );

        assert!(reply_markup.is_none());
        assert!(text.text.contains("可复制内容："));
        assert!(
            text.text
                .contains("复制查询命令：/lookup https://t.me/c/1/2 -100")
        );
        assert!(text.text.contains("打开转存消息：https://t.me/c/3/4"));
        assert!(!text.text.contains("d:r:all:8:1"));
    }

    // 发送响应只解析业务需要的轻量字段，避免完整 Message 里的复杂嵌套压垮 tokio worker 栈。
    #[test]
    fn test_parse_sent_message_lite_keeps_pending_state() {
        let response: serde_json::Value = serde_json::from_str(
            r#"{
                "@type": "message",
                "id": -9223372036854775808,
                "chat_id": 7814816521,
                "sending_state": {"@type": "messageSendingStatePending", "sending_id": "123"},
                "content": {"@type": "messageText", "text": {"text": "ignored", "entities": []}},
                "reply_markup": {"@type": "replyMarkupInlineKeyboard", "rows": []}
            }"#,
        )
        .unwrap();

        let message = parse_sent_message_lite(response).expect("lite message should parse");

        assert_eq!(message.id, -9223372036854775808);
        assert_eq!(message.chat_id, 7814816521);
        assert!(message.sending_state.is_some());
        assert!(message.reply_markup.is_none());
    }

    // TDLib 发送成功 update 会带回完整 message；这里覆盖 help-like reply markup 的解析。
    // 如果生成类型以后在这类结构上退化，本测试会先暴露问题。
    #[test]
    fn test_help_like_send_succeeded_update_can_deserialize() {
        let keyboard = build_inline_keyboard(vec![
            vec![build_copy_button(
                "复制 /transfer",
                "/transfer ",
                tdlib_rs::enums::ButtonStyle::Primary,
            )],
            vec![build_callback_button(
                "转存",
                "h:transfer",
                tdlib_rs::enums::ButtonStyle::Default,
            )],
        ]);
        let reply_markup =
            serde_json::to_value(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard)).unwrap();
        let mut response: serde_json::Value = serde_json::from_str(
            r#"{
                "@type": "updateMessageSendSucceeded",
                "message": {
                    "@type": "message",
                    "id": 100,
                    "sender_id": {"@type": "messageSenderUser", "user_id": 1},
                    "chat_id": 1,
                    "sending_state": null,
                    "scheduling_state": null,
                    "is_outgoing": true,
                    "is_pinned": false,
                    "is_from_offline": false,
                    "can_be_saved": true,
                    "has_timestamped_media": false,
                    "is_channel_post": false,
                    "is_paid_star_suggested_post": false,
                    "is_paid_ton_suggested_post": false,
                    "contains_unread_mention": false,
                    "date": 0,
                    "edit_date": 0,
                    "forward_info": null,
                    "import_info": null,
                    "interaction_info": null,
                    "unread_reactions": [],
                    "fact_check": null,
                    "suggested_post_info": null,
                    "reply_to": null,
                    "topic_id": null,
                    "self_destruct_type": null,
                    "self_destruct_in": 0.0,
                    "auto_delete_in": 0.0,
                    "via_bot_user_id": 0,
                    "sender_business_bot_user_id": 0,
                    "sender_boost_count": 0,
                    "sender_tag": "",
                    "paid_message_star_count": 0,
                    "author_signature": "",
                    "media_album_id": "0",
                    "effect_id": "0",
                    "restriction_info": null,
                    "summary_language_code": "",
                    "content": {
                        "@type": "messageText",
                        "text": {"text": "命令中心", "entities": []},
                        "link_preview": null,
                        "link_preview_options": null
                    },
                    "reply_markup": null
                },
                "old_message_id": -1
            }"#,
        )
        .unwrap();
        response["message"]["reply_markup"] = reply_markup;

        let update: tdlib_rs::enums::Update =
            serde_json::from_value(response).expect("send succeeded update should deserialize");
        assert!(matches!(
            update,
            tdlib_rs::enums::Update::MessageSendSucceeded(_)
        ));
    }
}
