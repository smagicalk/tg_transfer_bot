//! Telegram 按钮与内联键盘（Inline Keyboard）构建工具模块。
//!
//! # 核心职责
//! - 构造内联键盘结构体 `ReplyMarkupInlineKeyboard`。
//! - 构造一键复制文本按钮（`CopyText`）。
//! - 构造交互回调按钮（`Callback`），自动将短 Payload 进行 Base64 编码以适配 TDLib JSON 协议。
//! - 构造网页超链接按钮（`Url`）。
//! - 提供链接可直接在客户端打开的合规性校验。

use base64::{Engine as _, engine::general_purpose};

/// 构造一颗内联键盘（Inline Keyboard）。
///
/// # 参数
/// * `rows` - 按钮矩阵（外层为行，内层为该行中的各个按钮）
///
/// # 返回
/// 组装好的 `ReplyMarkupInlineKeyboard`
pub fn build_inline_keyboard(
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    tdlib_rs::types::ReplyMarkupInlineKeyboard { rows, force_reply: false }
}

/// 构造一键复制文本按钮。
///
/// Telegram 客户端用户点击该按钮后，系统会自动将 `value` 复制到剪贴板，适合命令代码、链接或任务标识。
///
/// # 参数
/// * `text` - 按钮表面显示的文案
/// * `value` - 点击后将被复制到系统剪贴板的文本
/// * `style` - 按钮视觉样式（如 Primary, Default, Danger）
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
            Box::new(tdlib_rs::types::InlineKeyboardButtonTypeCopyText {
                text: value.to_owned(),
            }),
        ),
    }
}

/// 构造交互式回调按钮（Callback Button）。
///
/// 点击后触发 TDLib 发送 `Update::NewCallbackQuery`，用于菜单跳转、面板切换或就地操作。
/// 业务层传入可读的短标识（如 `m:home`），此处统一 Base64 编码以适配 TDLib 二进制 Payload 规范。
///
/// # 参数
/// * `text` - 按钮表面文案
/// * `data` - 回调标识字符串（明文）
/// * `style` - 按钮视觉样式
pub fn build_callback_button(
    text: &str,
    data: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    let encoded_data = general_purpose::STANDARD.encode(data.as_bytes());
    tdlib_rs::types::InlineKeyboardButton {
        text: text.to_owned(),
        icon_custom_emoji_id: 0,
        style,
        r#type: tdlib_rs::enums::InlineKeyboardButtonType::Callback(
            Box::new(tdlib_rs::types::InlineKeyboardButtonTypeCallback { data: encoded_data }),
        ),
    }
}

/// 构造外部网页或应用跳转链接按钮。
///
/// # 参数
/// * `text` - 按钮文案
/// * `url` - 目标跳转 URL（如 `https://t.me/c/...`）
/// * `style` - 按钮视觉样式
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
            Box::new(tdlib_rs::types::InlineKeyboardButtonTypeUrl {
                url: url.to_owned(),
            }),
        ),
    }
}

/// 判断给定的链接是否适合配置在 Telegram 的 URL 按钮中。
///
/// 严格仅支持 HTTP/HTTPS 协议链接，避免使用客户端不支持直接点击的伪协议或深层链接。
///
/// # 参数
/// * `url` - 待检查的 URL 字符串
///
/// # 返回
/// 合法可打开返回 `true`，否则返回 `false`
pub fn is_openable_url(url: &str) -> bool {
    url.starts_with("https://") || url.starts_with("http://")
}

#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose};

    use super::{build_callback_button, is_openable_url};

    /// 验证回调按钮发送给 TDLib 前数据会被正确转为 Base64 编码。
    #[test]
    fn test_build_callback_button() {
        let button =
            build_callback_button("刷新", "j:st:42", tdlib_rs::enums::ButtonStyle::Primary);

        assert_eq!(button.text, "刷新");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = button.r#type else {
            panic!("button must be callback");
        };
        assert_eq!(
            general_purpose::STANDARD.decode(callback.data).unwrap(),
            b"j:st:42"
        );
    }

    /// 验证仅将标准的 HTTP/HTTPS 链接判定为可打开链接，拒绝 Telegram Deep Link。
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

