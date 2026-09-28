//! 响应式统一回复面板（ReplyPanel）构建模块。
//!
//! # 核心职责
//! - 聚合消息文本（Text）、内联按钮矩阵（Rows）以及渲染样式（Style）。
//! - 提供链式调用 API（如 `.row(...)`, `.rows(...)`），方便上层命令流畅组装复杂的交互卡片。
//! - 统一负责向 Telegram 发送（`send`）以及支持将面板拆解为正文和内联键盘对象（`into_card_parts`, `into_markdown_parts`）供后续原地局部刷新编辑。

use super::buttons::build_inline_keyboard;
use super::message::{
    send_card_message, send_card_message_with_buttons, send_copyable_message,
    send_copyable_message_with_buttons, send_markdown_message, send_markdown_message_with_buttons,
};

/// 回复面板正文的渲染风格枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplyPanelStyle {
    /// 标准 Telegram Markdown v1 渲染风格
    Markdown,
    /// 结构化卡片渲染风格（原生 FormattedText 转换，支持分区粗体、行内代码与超链接）
    Card,
    /// 等宽代码块可复制风格（整个正文包裹在 PreCode 中便于一键复制）
    Copyable,
}

/// 统一的 Telegram 回复面板构建器。
///
/// 封装正文文本、内联按钮行矩阵以及渲染风格。
pub struct ReplyPanel {
    /// 面板正文内容
    text: String,
    /// 内联键盘按钮矩阵
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    /// 渲染排版风格
    style: ReplyPanelStyle,
}

impl ReplyPanel {
    /// 构造 Markdown 渲染风格的面板。
    ///
    /// # 参数
    /// * `text` - Markdown 正文字符串
    pub fn markdown(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            rows: Vec::new(),
            style: ReplyPanelStyle::Markdown,
        }
    }

    /// 构造卡片渲染风格的面板。
    ///
    /// # 参数
    /// * `text` - 卡片排版格式文本
    pub fn card(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            rows: Vec::new(),
            style: ReplyPanelStyle::Card,
        }
    }

    /// 构造等宽纯文本可复制风格的面板。
    ///
    /// # 参数
    /// * `text` - 待完整复制的文本
    pub fn copyable(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            rows: Vec::new(),
            style: ReplyPanelStyle::Copyable,
        }
    }

    /// 追加单个内联键盘按钮行。
    ///
    /// # 参数
    /// * `buttons` - 包含一个或多个按钮的一维列表
    pub fn row(mut self, buttons: Vec<tdlib_rs::types::InlineKeyboardButton>) -> Self {
        self.rows.push(buttons);
        self
    }

    /// 批量追加多个内联键盘按钮行。
    ///
    /// # 参数
    /// * `rows` - 按钮行矩阵
    pub fn rows(mut self, rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>) -> Self {
        self.rows.extend(rows);
        self
    }

    /// 将当前面板发送至指定的 Telegram 对话。
    ///
    /// 根据配置的 `style` 与按钮行数量，自动调用对应的底层消息发送函数。
    ///
    /// # 参数
    /// * `chat_id` - 目标会话标识
    /// * `client_id` - 负责发送消息的 TDLib 客户端标识
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
            ReplyPanelStyle::Card => {
                if self.rows.is_empty() {
                    send_card_message(self.text, chat_id, client_id).await
                } else {
                    send_card_message_with_buttons(self.text, chat_id, self.rows, client_id).await
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

    /// 拆解面板并提取 Markdown 正文字符串与内联键盘结构体。
    ///
    /// 适合“首次发送消息后需记录 message_id，随后在原地编辑该消息”的场景。
    ///
    /// # 返回
    /// 提取出的 `(text, keyboard)` 元组，若风格非 Markdown 则报错
    pub fn into_markdown_parts(
        self,
    ) -> anyhow::Result<(String, tdlib_rs::types::ReplyMarkupInlineKeyboard)> {
        if self.style != ReplyPanelStyle::Markdown {
            anyhow::bail!("reply panel style is not markdown");
        }
        Ok((self.text, build_inline_keyboard(self.rows)))
    }

    /// 拆解面板并提取卡片正文字符串与内联键盘结构体。
    ///
    /// 适合菜单导航、任务详情等首次发送后再通过 callback 原地刷新的卡片场景。
    ///
    /// # 返回
    /// 提取出的 `(text, keyboard)` 元组，若风格非 Card 则报错
    pub fn into_card_parts(
        self,
    ) -> anyhow::Result<(String, tdlib_rs::types::ReplyMarkupInlineKeyboard)> {
        if self.style != ReplyPanelStyle::Card {
            anyhow::bail!("reply panel style is not card");
        }
        Ok((self.text, build_inline_keyboard(self.rows)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tgbot::send::build_copy_button;

    /// 验证 ReplyPanel 可以正确累加多行按钮并维护 Card 风格。
    #[test]
    fn test_reply_panel_collect_rows() {
        let panel = ReplyPanel::card("hello")
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
        assert_eq!(panel.style, ReplyPanelStyle::Card);
    }
}
