// `FormattedText` 构造工具。
// Telegram 的实体 offset/length 使用 UTF-16 单位，这里统一处理，避免调用方重复踩坑。

use crate::tgbot::TdError;

/// 构造 TDLib 文本消息内容。
pub(in crate::tgbot::send::message) fn build_text_input_message_content(
    text: tdlib_rs::types::FormattedText,
) -> tdlib_rs::enums::InputMessageContent {
    tdlib_rs::enums::InputMessageContent::InputMessageText(tdlib_rs::types::InputMessageText {
        text,
        link_preview_options: None,
        clear_draft: true,
    })
}

/// 构造不带实体的普通文本。
pub(in crate::tgbot::send::message) fn build_plain_formatted_text(
    text: String,
) -> tdlib_rs::types::FormattedText {
    tdlib_rs::types::FormattedText {
        text,
        entities: vec![],
    }
}

/// 解析 Markdown，转成 Telegram 原生 `FormattedText`。
pub(in crate::tgbot::send::message) async fn parse_markdown_text(
    text: String,
    client_id: i32,
) -> anyhow::Result<tdlib_rs::types::FormattedText> {
    let parsed = tdlib_rs::functions::parse_markdown(
        tdlib_rs::types::FormattedText {
            text,
            entities: vec![],
        },
        client_id,
    )
    .await
    .map_err(|e| anyhow::Error::new(TdError(e)))?;
    let tdlib_rs::enums::FormattedText::FormattedText(formatted_text) = parsed;
    Ok(formatted_text)
}

/// 构造整段可复制的等宽文本。
/// TDLib 的 `offset` 和 `length` 均按 UTF-16 code unit 计算。
pub(in crate::tgbot::send::message) fn build_copyable_formatted_text(
    text: String,
) -> anyhow::Result<tdlib_rs::types::FormattedText> {
    let length = i32::try_from(text.encode_utf16().count())
        .map_err(|_| anyhow::anyhow!("message too long"))?;

    Ok(tdlib_rs::types::FormattedText {
        text,
        entities: vec![tdlib_rs::types::TextEntity {
            offset: 0,
            length,
            r#type: tdlib_rs::enums::TextEntityType::PreCode(
                tdlib_rs::types::TextEntityTypePreCode {
                    language: "".to_owned(),
                },
            ),
        }],
    })
}
