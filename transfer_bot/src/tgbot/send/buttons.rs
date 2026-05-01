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
/// Telegram URL 按钮本身也能放 `tg://`，但旧版本用 TDLib 内部 message_id
/// 拼出的 `tg://openmessage` 在客户端里会出现“按钮能点但跳不到消息”的问题。
/// 因此业务层只把 TDLib `getMessageLink` 或 `t.me/c` 这类 HTTP(S) 链接当成可打开结果。
pub fn is_openable_url(url: &str) -> bool {
    url.starts_with("https://") || url.starts_with("http://")
}

#[cfg(test)]
mod tests {
    use super::is_openable_url;

    // 业务层只把 HTTP(S) 当成稳定结果链接，避免旧 tg://openmessage 再次进入打开按钮。
    #[test]
    fn test_is_openable_url_rejects_telegram_deep_link() {
        assert!(is_openable_url("https://t.me/c/5106953357/734"));
        assert!(is_openable_url("http://example.com/message"));
        assert!(!is_openable_url(
            "tg://openmessage?chat_id=-5106953357&message_id=769654784"
        ));
        assert!(!is_openable_url("chat_id=-5106953357 message_id=769654784"));
    }
}
