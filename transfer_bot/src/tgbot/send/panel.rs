// 统一回复面板。
// 命令层可以通过面板统一组合正文、按钮和发送风格。

use super::buttons::build_inline_keyboard;
use super::message::{
    send_copyable_message, send_copyable_message_with_buttons, send_markdown_message,
    send_markdown_message_with_buttons,
};

/// 回复文本的渲染风格。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplyPanelStyle {
    Markdown,
    Copyable,
}

/// 统一的 TG 回复面板。
///
/// 用途：
/// - 统一维护正文文本
/// - 统一维护按钮行
/// - 根据需要选择 Markdown 或等宽复制风格
pub struct ReplyPanel {
    text: String,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    style: ReplyPanelStyle,
}

impl ReplyPanel {
    /// 构造 Markdown 风格面板。
    pub fn markdown(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            rows: Vec::new(),
            style: ReplyPanelStyle::Markdown,
        }
    }

    /// 构造等宽可复制风格面板。
    pub fn copyable(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            rows: Vec::new(),
            style: ReplyPanelStyle::Copyable,
        }
    }

    /// 追加一整行按钮。
    pub fn row(mut self, buttons: Vec<tdlib_rs::types::InlineKeyboardButton>) -> Self {
        self.rows.push(buttons);
        self
    }

    /// 追加多行按钮。
    pub fn rows(mut self, rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>) -> Self {
        self.rows.extend(rows);
        self
    }

    /// 发送当前面板。
    pub async fn send(self, chat_id: i64, client_id: i32) -> anyhow::Result<()> {
        match self.style {
            ReplyPanelStyle::Markdown => {
                if self.rows.is_empty() {
                    send_markdown_message(self.text, chat_id, client_id).await
                } else {
                    send_markdown_message_with_buttons(self.text, chat_id, self.rows, client_id)
                        .await
                }
            }
            ReplyPanelStyle::Copyable => {
                if self.rows.is_empty() {
                    send_copyable_message(self.text, chat_id, client_id).await
                } else {
                    send_copyable_message_with_buttons(self.text, chat_id, self.rows, client_id)
                        .await
                }
            }
        }
    }

    /// 拆出 Markdown 文本与 inline keyboard。
    ///
    /// 适合像 `/downloads` 这种“首次发送 + 后续编辑同一条消息”的场景。
    pub fn into_markdown_parts(
        self,
    ) -> anyhow::Result<(String, tdlib_rs::types::ReplyMarkupInlineKeyboard)> {
        if self.style != ReplyPanelStyle::Markdown {
            anyhow::bail!("reply panel style is not markdown");
        }
        Ok((self.text, build_inline_keyboard(self.rows)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tgbot::send::build_copy_button;

    // ReplyPanel 应能正确累积按钮行。
    #[test]
    fn test_reply_panel_collect_rows() {
        let panel = ReplyPanel::markdown("hello")
            .row(vec![build_copy_button(
                "复制",
                "value",
                tdlib_rs::enums::ButtonStyle::Default,
            )])
            .row(vec![build_copy_button(
                "复制2",
                "value2",
                tdlib_rs::enums::ButtonStyle::Primary,
            )]);

        assert_eq!(panel.rows.len(), 2);
        assert_eq!(panel.style, ReplyPanelStyle::Markdown);
    }
}
