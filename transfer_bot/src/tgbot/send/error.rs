//! 统一交互式错误卡片（Interaction Error Card）模块。
//!
//! # 核心职责
//! - 提供在原地编辑失败时自动回退为发送独立错误消息的 `edit_interaction_card_or_error`。
//! - 统一定义错误卡片的版式：标题、状态标签（`failed`）、原因、可折叠原始错误代码块及恢复动作按钮行（“查看命令”、“主菜单”、“复制错误”）。

use super::{
    ReplyPanel, build_callback_button, build_copy_button, edit_card_message_with_inline_keyboard,
};
use crate::tgbot::transfer::card;

/// 编辑交互卡片消息；若原消息编辑失败（如消息已过期或被删），则自动发送独立错误卡片。
///
/// 在 Callback Query 已被 ACK 应答后，Telegram 客户端不会再展示第二次弹窗提示；
/// 因此当原地编辑失败时必须发送一条新消息，否则用户端只会看到按钮转圈结束而界面无任何响应。
///
/// # 参数
/// * `text` - 目标卡片正文文本
/// * `chat_id` - 目标会话标识
/// * `message_id` - 待编辑的消息 ID
/// * `keyboard` - 附带的内联键盘
/// * `client_id` - 客户端标识
/// * `title` - 错误备选标题
/// * `detail` - 错误详细提示
///
/// # 返回
/// 成功完成编辑返回 `Ok(())`，编辑失败且已发送错误卡片后返回原 Err
pub async fn edit_interaction_card_or_error(
    text: String,
    chat_id: i64,
    message_id: i64,
    keyboard: tdlib_rs::types::ReplyMarkupInlineKeyboard,
    client_id: i32,
    title: &str,
    detail: &str,
) -> anyhow::Result<()> {
    if let Err(err) =
        edit_card_message_with_inline_keyboard(text, chat_id, message_id, keyboard, client_id).await
    {
        send_interaction_error_card(chat_id, client_id, title, detail, &err).await?;
        return Err(err);
    }
    Ok(())
}

/// 发送统一排版的交互错误卡片。
///
/// 调用方仅需提供错误简题、说明和底层错误对象；本函数自动格式化为标准卡片并附加恢复操作按钮行。
///
/// # 参数
/// * `request_chat_id` - 目标会话 ID
/// * `client_id` - 响应客户端标识
/// * `title` - 卡片标题
/// * `detail` - 错误原因与排查说明
/// * `err` - 原始错误对象
pub async fn send_interaction_error_card(
    request_chat_id: i64,
    client_id: i32,
    title: &str,
    detail: &str,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    let error_text = err.to_string();
    let result = ReplyPanel::card(interaction_error_text(title, detail, &error_text))
        .row(interaction_error_action_row(&error_text))
        .send(request_chat_id, client_id)
        .await;
    if let Err(send_err) = &result {
        tracing::warn!(
            request_chat_id,
            client_id,
            title,
            error = %send_err,
            "send interaction error card failed"
        );
    }
    result
}

/// 构造统一的交互错误卡片富文本正文。
///
/// 包含标题、状态（`failed`）、原因和原始错误代码块。
///
/// # 参数
/// * `title` - 标题
/// * `detail` - 说明
/// * `error_text` - 错误内容
fn interaction_error_text(title: &str, detail: &str, error_text: &str) -> String {
    [
        title.to_owned(),
        format!("状态：{}", card::code("failed")),
        card::DIVIDER.to_owned(),
        card::section("原因"),
        card::note(detail),
        card::section("错误"),
        card::pre_code(error_text),
    ]
    .join("\n")
}

/// 构造统一的“复制错误”按钮行（测试专用）。
#[cfg(test)]
fn copy_error_row(error_text: &str) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![build_copy_button(
        "复制错误",
        error_text,
        tdlib_rs::enums::ButtonStyle::Default,
    )]
}

/// 构建交互错误卡片底部的操作按钮行（“查看命令”、“主菜单”、“复制错误”）。
fn interaction_error_action_row(error_text: &str) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![
        crate::tgbot::transfer::build_view_commands_button(None),
        build_callback_button(
            "菜单",
            &crate::tgbot::transfer::build_menu_home_button_data_for_outer(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        build_copy_button(
            "复制错误",
            error_text,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::{copy_error_row, interaction_error_action_row, interaction_error_text};

    /// 验证交互错误卡片文本布局包含状态、原因和错误各区域。
    #[test]
    fn test_interaction_error_text_layout() {
        let text = interaction_error_text("帮助刷新失败", "帮助页未更新。", "db timeout");

        assert!(text.contains("帮助刷新失败"));
        assert!(text.contains("状态：‹failed›"));
        assert!(text.contains("■ 原因"));
        assert!(text.contains("■ 错误"));
    }

    /// 验证复制错误按钮行仅生成单一按钮。
    #[test]
    fn test_copy_error_row_layout() {
        let row = copy_error_row("db timeout");

        assert_eq!(row.len(), 1);
        assert_eq!(row[0].text, "复制错误");
    }

    /// 验证错误动作行包含完整的三个恢复入口。
    #[test]
    fn test_interaction_error_action_row_has_recovery_entries() {
        let row = interaction_error_action_row("db timeout");

        assert_eq!(row.len(), 3);
        assert_eq!(row[0].text, "查看命令");
        assert_eq!(row[1].text, "菜单");
        assert_eq!(row[2].text, "复制错误");
    }
}
