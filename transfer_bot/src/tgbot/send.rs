//! Telegram 消息发送与交互式界面渲染门面模块。
//!
//! # 核心职责
//! - **按钮与键盘构建（`buttons`）**：构建内联回调按钮（Callback Button）、一键复制按钮（Copy Button）、超链接按钮（URL Button）以及原生选择器。
//! - **消息发送与编辑（`message`）**：封装纯文本、卡片风格、Markdown 等格式的消息发送、编辑与异步回执等待。
//! - **响应式面板（`panel`）**：提供类似 UI 组件的 `ReplyPanel`，统一组装正文、按钮矩阵与渲染样式。
//! - **错误统一展示（`error`）**：统一封装带重试或恢复建议的交互式错误卡片。

mod buttons;
mod error;
mod message;
mod panel;

pub use buttons::{
    build_callback_button, build_copy_button, build_inline_keyboard, build_url_button,
    is_openable_url,
};
pub use error::{edit_interaction_card_or_error, send_interaction_error_card};
pub use message::{
    SentMessageReceipt, answer_callback_query, delete_chat_reply_markup, delete_message,
    edit_card_message_with_inline_keyboard, edit_local_photo,
    edit_markdown_message_with_inline_keyboard, observe_message_send_failed_for_client,
    observe_message_send_succeeded_for_client, send_card_message, send_card_message_replying_to,
    send_card_message_with_buttons, send_card_message_with_buttons_replying_to,
    send_card_message_with_buttons_returning, send_card_message_with_force_reply_returning,
    send_card_message_with_remove_keyboard,
    send_card_message_with_target_chat_request_keyboard_returning,
    send_card_message_with_user_request_keyboard_returning, send_copyable_message,
    send_copyable_message_with_buttons, send_error_message, send_local_photo_returning,
    send_markdown_message, send_markdown_message_with_buttons,
    send_markdown_message_with_buttons_returning, send_markdown_message_with_inline_keyboard,
    send_text_message, set_reply_markup_enabled, wait_for_sent_message, wait_for_sent_message_id,
    wait_for_sent_message_with_timeout,
};
pub use panel::{ReplyPanel, ReplyPanelStyle};

