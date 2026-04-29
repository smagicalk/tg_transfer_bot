// 发送消息工具入口：
// - `buttons`：构造 inline keyboard / copy / url 按钮
// - `message`：发送、编辑消息和应答 callback
// - `panel`：命令层常用的统一回复面板

mod buttons;
mod message;
mod panel;

pub use buttons::{build_copy_button, build_inline_keyboard, build_url_button};
pub use message::{
    answer_callback_query, edit_markdown_message_with_inline_keyboard, send_copyable_message,
    send_copyable_message_with_buttons, send_error_message, send_markdown_message,
    send_markdown_message_with_buttons, send_markdown_message_with_buttons_returning,
    send_markdown_message_with_inline_keyboard, send_text_message,
};
pub use panel::{ReplyPanel, ReplyPanelStyle};
