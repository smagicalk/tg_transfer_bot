// Telegram inline keyboard 构建工具。
// 这里不发送消息，只负责生成按钮和键盘结构。

/// 构造一颗 inline keyboard。
pub fn build_inline_keyboard(
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    tdlib_rs::types::ReplyMarkupInlineKeyboard { rows }
}

/// 构造复制文本按钮。
///
/// Telegram 客户端点击后会直接复制指定文本，适合命令、链接、job_id 这类内容。
pub fn build_copy_button(
    text: &str,
    value: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    tdlib_rs::types::InlineKeyboardButton {
        text: text.to_owned(),
        icon_custom_emoji_id: 0,
        style,
        r#type: tdlib_rs::enums::InlineKeyboardButtonType::CopyText(
            tdlib_rs::types::InlineKeyboardButtonTypeCopyText {
                text: value.to_owned(),
            },
        ),
    }
}

/// 构造打开链接按钮。
pub fn build_url_button(
    text: &str,
    url: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    tdlib_rs::types::InlineKeyboardButton {
        text: text.to_owned(),
        icon_custom_emoji_id: 0,
        style,
        r#type: tdlib_rs::enums::InlineKeyboardButtonType::Url(
            tdlib_rs::types::InlineKeyboardButtonTypeUrl {
                url: url.to_owned(),
            },
        ),
    }
}

/// 判断链接是否适合放到 Telegram URL 按钮里。
///
/// TDLib 的 `getMessageLink` 成功时会返回 HTTPS 链接；`tg://openmessage`
/// 这类客户端 deeplink 在部分 chat 或客户端里不会跳转，所以结果卡片不再把它当成可打开链接。
pub fn is_openable_url(url: &str) -> bool {
    url.starts_with("https://") || url.starts_with("http://")
}
