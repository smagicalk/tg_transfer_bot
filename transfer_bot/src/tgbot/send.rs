// 发送消息工具入口：
// - `buttons`：构造 inline keyboard / copy / url 按钮
// - `message`：发送、编辑消息和应答 callback
// - `panel`：命令层常用的统一回复面板
// - `error`：统一交互错误卡片

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
    answer_callback_query, edit_card_message_with_inline_keyboard,
    edit_markdown_message_with_inline_keyboard, observe_message_send_failed_for_client,
    observe_message_send_succeeded_for_client, send_card_message, send_card_message_with_buttons,
    send_card_message_with_buttons_returning, send_card_message_with_force_reply_returning,
    send_card_message_with_remove_keyboard, send_copyable_message,
    send_copyable_message_with_buttons, send_error_message, send_markdown_message,
    send_markdown_message_with_buttons, send_markdown_message_with_buttons_returning,
    send_markdown_message_with_inline_keyboard, send_text_message, set_reply_markup_enabled,
    wait_for_sent_message, wait_for_sent_message_id,
};
pub use panel::{ReplyPanel, ReplyPanelStyle};
