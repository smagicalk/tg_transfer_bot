// Telegram 消息发送工具入口。
// 高层 API 保留在这里，TDLib 请求细节和 `FormattedText` 构造拆到子模块。

mod content;
mod raw;
mod state;

use std::sync::atomic::{AtomicBool, Ordering};

use super::buttons::build_inline_keyboard;
use content::{
    build_card_formatted_text, build_copyable_formatted_text, build_plain_formatted_text,
    parse_markdown_text,
};
use raw::{send_formatted_text_message, send_formatted_text_message_returning};

pub use raw::{
    answer_callback_query, edit_card_message_with_inline_keyboard,
    edit_markdown_message_with_inline_keyboard,
};
pub use state::{
    observe_message_send_failed_for_client, observe_message_send_succeeded_for_client,
    wait_for_sent_message, wait_for_sent_message_id,
};

/// 当前账号是否支持发送 reply_markup。
///
/// TDLib 文档将 `sendMessage.reply_markup` 标注为 bot 能力；用户号登录时即使请求里带了按钮，
/// Telegram 客户端也不会稳定展示。默认开启，主程序读取配置后会按登录模式覆盖。
static REPLY_MARKUP_ENABLED: AtomicBool = AtomicBool::new(true);

/// 设置当前发送层是否允许携带 reply_markup。
pub fn set_reply_markup_enabled(enabled: bool) {
    REPLY_MARKUP_ENABLED.store(enabled, Ordering::Relaxed);
    tracing::info!(enabled, "tdlib reply markup capability configured");
}

/// 查询当前发送层是否允许携带 reply_markup。
pub(in crate::tgbot::send) fn is_reply_markup_enabled() -> bool {
    REPLY_MARKUP_ENABLED.load(Ordering::Relaxed)
}

/// 向指定 chat 发送纯文本消息。
pub async fn send_text_message(text: String, chat_id: i64, client_id: i32) -> anyhow::Result<()> {
    send_formatted_text_message(build_plain_formatted_text(text), chat_id, None, client_id).await
}

/// 向指定 chat 发送 Markdown 文本。
/// 适合“说明文字 + 命令示例”这种场景，命令可用反引号包成代码格式。
pub async fn send_markdown_message(
    text: String,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let formatted_text = parse_markdown_text(text, client_id).await?;
    send_formatted_text_message(formatted_text, chat_id, None, client_id).await
}

/// 向指定 chat 发送 Markdown 文本并附带 inline keyboard。
/// 适合分页列表、命令面板等需要“原地翻页”的场景。
pub async fn send_markdown_message_with_inline_keyboard(
    text: String,
    chat_id: i64,
    keyboard: tdlib_rs::types::ReplyMarkupInlineKeyboard,
    client_id: i32,
) -> anyhow::Result<()> {
    let formatted_text = parse_markdown_text(text, client_id).await?;
    send_formatted_text_message(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard)),
        client_id,
    )
    .await
}

/// 向指定 chat 发送 Markdown 文本并附带 inline keyboard，同时返回消息对象。
/// `/transfer` 进度面板依赖返回的 message_id 执行后续编辑。
pub async fn send_markdown_message_with_buttons_returning(
    text: String,
    chat_id: i64,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    client_id: i32,
) -> anyhow::Result<tdlib_rs::types::Message> {
    let formatted_text = parse_markdown_text(text, client_id).await?;
    send_formatted_text_message_returning(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(
            build_inline_keyboard(rows),
        )),
        client_id,
    )
    .await
}

/// 向指定 chat 发送 Markdown 文本并使用按钮行配置键盘。
/// 这是上层命令最常用的入口，避免每个模块都手动构造 `ReplyMarkupInlineKeyboard`。
pub async fn send_markdown_message_with_buttons(
    text: String,
    chat_id: i64,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    client_id: i32,
) -> anyhow::Result<()> {
    send_markdown_message_with_inline_keyboard(
        text,
        chat_id,
        build_inline_keyboard(rows),
        client_id,
    )
    .await
}

/// 向指定 chat 发送卡片风格文本。
///
/// 卡片文本会在本地转成 TDLib 原生 `FormattedText`，不经过 Markdown 解析。
pub async fn send_card_message(text: String, chat_id: i64, client_id: i32) -> anyhow::Result<()> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message(formatted_text, chat_id, None, client_id).await
}

/// 向指定 chat 发送卡片风格文本并附带按钮。
pub async fn send_card_message_with_buttons(
    text: String,
    chat_id: i64,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    client_id: i32,
) -> anyhow::Result<()> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(
            build_inline_keyboard(rows),
        )),
        client_id,
    )
    .await
}

/// 向指定 chat 发送卡片风格文本并附带按钮，同时返回消息对象。
pub async fn send_card_message_with_buttons_returning(
    text: String,
    chat_id: i64,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    client_id: i32,
) -> anyhow::Result<tdlib_rs::types::Message> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message_returning(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(
            build_inline_keyboard(rows),
        )),
        client_id,
    )
    .await
}

/// 向指定 chat 发送卡片风格文本并触发 ForceReply 输入框。
///
/// ForceReply 不能和 inline keyboard 同时存在，因此这里只负责“要求用户回复输入”这一类场景。
pub async fn send_card_message_with_force_reply_returning(
    text: String,
    chat_id: i64,
    placeholder: &str,
    client_id: i32,
) -> anyhow::Result<tdlib_rs::types::Message> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message_returning(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::ForceReply(
            tdlib_rs::types::ReplyMarkupForceReply {
                is_personal: true,
                input_field_placeholder: placeholder.chars().take(64).collect(),
            },
        )),
        client_id,
    )
    .await
}

/// 向指定 chat 发送便于复制的等宽文本，并附带按钮。
/// 适合错误详情、诊断信息这类“主体要复制，附加动作也要点”的场景。
pub async fn send_copyable_message_with_buttons(
    text: String,
    chat_id: i64,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    client_id: i32,
) -> anyhow::Result<()> {
    send_formatted_text_message(
        build_copyable_formatted_text(text)?,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(
            build_inline_keyboard(rows),
        )),
        client_id,
    )
    .await
}

/// 向指定 chat 发送便于整段复制的等宽文本。
/// 使用 TDLib 的 `textEntityTypePreCode` 包裹整段消息。
pub async fn send_copyable_message(
    text: String,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    send_formatted_text_message(
        build_copyable_formatted_text(text)?,
        chat_id,
        None,
        client_id,
    )
    .await
}

/// 发送错误信息（统一转成字符串后发送）。
pub async fn send_error_message(
    error: anyhow::Error,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    send_copyable_message(error.to_string(), chat_id, client_id).await
}
