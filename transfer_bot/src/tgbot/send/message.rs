// Telegram 消息发送工具入口。
// 高层 API 保留在这里，TDLib 请求细节和 `FormattedText` 构造拆到子模块。

mod content;
mod raw;
mod state;

use super::buttons::build_inline_keyboard;
use content::{
    build_card_formatted_text, build_copyable_formatted_text, build_plain_formatted_text,
    parse_markdown_text,
};
use raw::{
    send_formatted_text_message, send_formatted_text_message_returning,
    send_formatted_text_message_returning_with_reply_to,
};

pub use raw::{
    answer_callback_query, edit_card_message_with_inline_keyboard,
    edit_markdown_message_with_inline_keyboard,
};
pub use state::{
    SentMessageReceipt, observe_message_send_failed_for_client,
    observe_message_send_succeeded_for_client, wait_for_sent_message, wait_for_sent_message_id,
    wait_for_sent_message_with_timeout,
};

/// 设置当前发送层是否允许携带 reply_markup。
pub fn set_reply_markup_enabled(enabled: bool) {
    crate::app_context::app_context()
        .send_capabilities
        .set_reply_markup_enabled(enabled);
    tracing::info!(enabled, "tdlib reply markup capability configured");
}

/// 查询当前发送层是否允许携带 reply_markup。
pub(in crate::tgbot::send) fn is_reply_markup_enabled() -> bool {
    crate::app_context::app_context()
        .send_capabilities
        .reply_markup_enabled()
}

/// 向指定 chat 发送纯文本消息。
pub async fn send_text_message(text: String, chat_id: i64, client_id: i32) -> anyhow::Result<()> {
    send_formatted_text_message(build_plain_formatted_text(text), chat_id, None, client_id).await
}

/// 向指定 chat 发送 Markdown 文本。
/// 适合“说明文字 + 命令示例”这种场景，命令可用反引号包成代码格式。
pub async fn send_markdown_message(
    text: String,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let formatted_text = parse_markdown_text(text, client_id).await?;
    send_formatted_text_message(formatted_text, chat_id, None, client_id).await
}

/// 向指定 chat 发送 Markdown 文本并附带 inline keyboard。
/// 适合分页列表、命令面板等需要“原地翻页”的场景。
pub async fn send_markdown_message_with_inline_keyboard(
    text: String,
    chat_id: i64,
    keyboard: tdlib_rs::types::ReplyMarkupInlineKeyboard,
    client_id: i32,
) -> anyhow::Result<()> {
    let formatted_text = parse_markdown_text(text, client_id).await?;
    send_formatted_text_message(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(keyboard)),
        client_id,
    )
    .await
}

/// 向指定 chat 发送 Markdown 文本并附带 inline keyboard，同时返回消息对象。
/// `/transfer` 进度面板依赖返回的 message_id 执行后续编辑。
pub async fn send_markdown_message_with_buttons_returning(
    text: String,
    chat_id: i64,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    client_id: i32,
) -> anyhow::Result<SentMessageReceipt> {
    let formatted_text = parse_markdown_text(text, client_id).await?;
    send_formatted_text_message_returning(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(
            build_inline_keyboard(rows),
        )),
        client_id,
    )
    .await
}

/// 向指定 chat 发送 Markdown 文本并使用按钮行配置键盘。
/// 这是上层命令最常用的入口，避免每个模块都手动构造 `ReplyMarkupInlineKeyboard`。
pub async fn send_markdown_message_with_buttons(
    text: String,
    chat_id: i64,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    client_id: i32,
) -> anyhow::Result<()> {
    send_markdown_message_with_inline_keyboard(
        text,
        chat_id,
        build_inline_keyboard(rows),
        client_id,
    )
    .await
}

/// 向指定 chat 发送卡片风格文本。
///
/// 卡片文本会在本地转成 TDLib 原生 `FormattedText`，不经过 Markdown 解析。
pub async fn send_card_message(text: String, chat_id: i64, client_id: i32) -> anyhow::Result<()> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message(formatted_text, chat_id, None, client_id).await
}

/// 向指定 chat 发送卡片风格文本，并要求客户端移除自定义 reply keyboard。
///
/// Telegram 的原生“选择聊天”按钮属于 reply keyboard，和 inline keyboard 不是同一类控件。
/// 选择完成、取消或过期时发送这个消息，能避免输入框下方残留旧的“选择聊天”按钮。
pub async fn send_card_message_with_remove_keyboard(
    text: String,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::RemoveKeyboard(
            tdlib_rs::types::ReplyMarkupRemoveKeyboard { is_personal: true },
        )),
        client_id,
    )
    .await
}

/// 删除 bot 自己发送的单条流程提示。
///
/// 交互向导会在阶段切换时产生新的卡片；调用方只应传入由发送 API 返回的
/// bot 消息 ID。删除失败不应阻断主流程，由上层决定是否记录并继续。
pub async fn delete_message(chat_id: i64, message_id: i64, client_id: i32) -> anyhow::Result<()> {
    if message_id <= 0 {
        anyhow::bail!("invalid message id: {message_id}");
    }
    tdlib_rs::functions::delete_messages(chat_id, vec![message_id], true, client_id)
        .await
        .map_err(|error| anyhow::anyhow!("delete message failed: {}", error.message))
}

/// 删除指定消息上的默认 reply markup（原生选聊/ForceReply）。
///
/// TDLib 要求先调用这个接口再删除承载键盘的消息，否则客户端可能继续显示旧键盘。
pub async fn delete_chat_reply_markup(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    if message_id <= 0 {
        anyhow::bail!("invalid reply markup message id: {message_id}");
    }
    tdlib_rs::functions::delete_chat_reply_markup(chat_id, message_id, client_id)
        .await
        .map_err(|error| anyhow::anyhow!("delete chat reply markup failed: {}", error.message))
}

/// 向指定 chat 发送卡片风格文本并附带按钮。
pub async fn send_card_message_with_buttons(
    text: String,
    chat_id: i64,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    client_id: i32,
) -> anyhow::Result<()> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(
            build_inline_keyboard(rows),
        )),
        client_id,
    )
    .await
}

/// 发送卡片风格文本和按钮，并回复一条已转存的目标消息。
///
/// 同聊天使用普通回复，跨聊天使用 TDLib 的 external message reply。
pub async fn send_card_message_with_buttons_replying_to(
    text: String,
    chat_id: i64,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    target_chat_id: i64,
    target_message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message_returning_with_reply_to(
        formatted_text,
        chat_id,
        Some(build_message_reply_to(
            chat_id,
            target_chat_id,
            target_message_id,
        )),
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(
            build_inline_keyboard(rows),
        )),
        client_id,
    )
    .await
    .map(|_| ())
}

/// 发送卡片风格文本，并回复一条已转存的目标消息。
pub async fn send_card_message_replying_to(
    text: String,
    chat_id: i64,
    target_chat_id: i64,
    target_message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message_returning_with_reply_to(
        formatted_text,
        chat_id,
        Some(build_message_reply_to(
            chat_id,
            target_chat_id,
            target_message_id,
        )),
        None,
        client_id,
    )
    .await
    .map(|_| ())
}

/// 向指定 chat 发送卡片风格文本并附带按钮，同时返回消息对象。
pub async fn send_card_message_with_buttons_returning(
    text: String,
    chat_id: i64,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    client_id: i32,
) -> anyhow::Result<SentMessageReceipt> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message_returning(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(
            build_inline_keyboard(rows),
        )),
        client_id,
    )
    .await
}

/// 向指定 chat 发送卡片风格文本并触发 ForceReply 输入框。
///
/// ForceReply 不能和 inline keyboard 同时存在，因此这里只负责“要求用户回复输入”这一类场景。
pub async fn send_card_message_with_force_reply_returning(
    text: String,
    chat_id: i64,
    placeholder: &str,
    client_id: i32,
) -> anyhow::Result<SentMessageReceipt> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message_returning(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::ForceReply(
            tdlib_rs::types::ReplyMarkupForceReply {
                is_personal: true,
                input_field_placeholder: placeholder.chars().take(64).collect(),
            },
        )),
        client_id,
    )
    .await
}

/// 向指定私聊发送 Telegram 原生目标聊天选择器。
pub async fn send_card_message_with_target_chat_request_keyboard_returning(
    text: String,
    chat_id: i64,
    group_button_id: i32,
    channel_button_id: i32,
    client_id: i32,
) -> anyhow::Result<SentMessageReceipt> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message_returning(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::ShowKeyboard(
            build_target_chat_request_keyboard(group_button_id, channel_button_id, "选择目标聊天"),
        )),
        client_id,
    )
    .await
}

/// 向指定私聊发送 Telegram 原生用户选择器。
///
/// `keyboardButtonTypeRequestUsers` 只能在 bot 私聊中使用；调用方负责在发送前
/// 记录当前等待的业务动作，并在 `messageUsersShared` 到达后消费它。
pub async fn send_card_message_with_user_request_keyboard_returning(
    text: String,
    chat_id: i64,
    button_id: i32,
    client_id: i32,
) -> anyhow::Result<SentMessageReceipt> {
    let formatted_text = build_card_formatted_text(text)?;
    send_formatted_text_message_returning(
        formatted_text,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::ShowKeyboard(
            build_user_request_keyboard(button_id, "选择要授权的用户"),
        )),
        client_id,
    )
    .await
}

fn build_target_chat_request_keyboard(
    group_button_id: i32,
    channel_button_id: i32,
    placeholder: &str,
) -> tdlib_rs::types::ReplyMarkupShowKeyboard {
    let channel_rights = tdlib_rs::types::ChatAdministratorRights {
        can_manage_chat: true,
        can_post_messages: true,
        ..Default::default()
    };
    let request_button = |text: &str,
                          id: i32,
                          chat_is_channel: bool,
                          user_administrator_rights,
                          bot_administrator_rights,
                          bot_is_member| tdlib_rs::types::KeyboardButton {
        text: text.to_owned(),
        icon_custom_emoji_id: 0,
        style: tdlib_rs::enums::ButtonStyle::Primary,
        r#type: tdlib_rs::enums::KeyboardButtonType::RequestChat(
            tdlib_rs::types::KeyboardButtonTypeRequestChat {
                id,
                chat_is_channel,
                restrict_chat_is_forum: false,
                chat_is_forum: false,
                restrict_chat_has_username: false,
                chat_has_username: false,
                chat_is_created: false,
                user_administrator_rights,
                bot_administrator_rights,
                bot_is_member,
                request_title: true,
                request_username: true,
                request_photo: false,
            },
        ),
    };
    let group_button = request_button("选择群组", group_button_id, false, None, None, true);
    let channel_button = request_button(
        "选择频道",
        channel_button_id,
        true,
        Some(channel_rights.clone()),
        Some(channel_rights),
        false,
    );
    let cancel_button = tdlib_rs::types::KeyboardButton {
        text: "取消".to_owned(),
        icon_custom_emoji_id: 0,
        style: tdlib_rs::enums::ButtonStyle::Danger,
        r#type: tdlib_rs::enums::KeyboardButtonType::Text,
    };
    // 原生选聊不是所有客户端都能顺利筛出目标；保留一个文字按钮，
    // 让用户可以无缝切换到 ForceReply 手动输入，而不用重新返回菜单。
    let manual_input_button = tdlib_rs::types::KeyboardButton {
        text: "手动输入目标".to_owned(),
        icon_custom_emoji_id: 0,
        style: tdlib_rs::enums::ButtonStyle::Default,
        r#type: tdlib_rs::enums::KeyboardButtonType::Text,
    };

    tdlib_rs::types::ReplyMarkupShowKeyboard {
        rows: vec![
            vec![group_button, channel_button],
            vec![manual_input_button],
            vec![cancel_button],
        ],
        is_persistent: false,
        resize_keyboard: true,
        one_time: true,
        is_personal: true,
        input_field_placeholder: placeholder.chars().take(64).collect(),
    }
}

fn build_user_request_keyboard(
    button_id: i32,
    placeholder: &str,
) -> tdlib_rs::types::ReplyMarkupShowKeyboard {
    let request_user_button = tdlib_rs::types::KeyboardButton {
        text: "选择 Telegram 用户".to_owned(),
        icon_custom_emoji_id: 0,
        style: tdlib_rs::enums::ButtonStyle::Primary,
        r#type: tdlib_rs::enums::KeyboardButtonType::RequestUsers(
            tdlib_rs::types::KeyboardButtonTypeRequestUsers {
                id: button_id,
                restrict_user_is_bot: true,
                user_is_bot: false,
                restrict_user_is_premium: false,
                user_is_premium: false,
                max_quantity: 1,
                request_name: true,
                request_username: true,
                request_photo: false,
            },
        ),
    };
    let cancel_button = tdlib_rs::types::KeyboardButton {
        text: "取消".to_owned(),
        icon_custom_emoji_id: 0,
        style: tdlib_rs::enums::ButtonStyle::Danger,
        r#type: tdlib_rs::enums::KeyboardButtonType::Text,
    };

    tdlib_rs::types::ReplyMarkupShowKeyboard {
        rows: vec![vec![request_user_button], vec![cancel_button]],
        is_persistent: false,
        resize_keyboard: true,
        one_time: true,
        is_personal: true,
        input_field_placeholder: placeholder.chars().take(64).collect(),
    }
}

/// 构造指向已转存消息的 Telegram 原生回复锚点。
fn build_message_reply_to(
    send_chat_id: i64,
    target_chat_id: i64,
    target_message_id: i64,
) -> tdlib_rs::enums::InputMessageReplyTo {
    if send_chat_id == target_chat_id {
        return tdlib_rs::enums::InputMessageReplyTo::Message(
            tdlib_rs::types::InputMessageReplyToMessage {
                message_id: target_message_id,
                quote: None,
                checklist_task_id: 0,
            },
        );
    }

    tdlib_rs::enums::InputMessageReplyTo::ExternalMessage(
        tdlib_rs::types::InputMessageReplyToExternalMessage {
            chat_id: target_chat_id,
            message_id: target_message_id,
            quote: None,
            checklist_task_id: 0,
        },
    )
}

/// 向指定 chat 发送便于复制的等宽文本，并附带按钮。
/// 适合错误详情、诊断信息这类“主体要复制，附加动作也要点”的场景。
pub async fn send_copyable_message_with_buttons(
    text: String,
    chat_id: i64,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    client_id: i32,
) -> anyhow::Result<()> {
    send_formatted_text_message(
        build_copyable_formatted_text(text)?,
        chat_id,
        Some(tdlib_rs::enums::ReplyMarkup::InlineKeyboard(
            build_inline_keyboard(rows),
        )),
        client_id,
    )
    .await
}

/// 向指定 chat 发送便于整段复制的等宽文本。
/// 使用 TDLib 的 `textEntityTypePreCode` 包裹整段消息。
pub async fn send_copyable_message(
    text: String,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    send_formatted_text_message(
        build_copyable_formatted_text(text)?,
        chat_id,
        None,
        client_id,
    )
    .await
}

/// 发送错误信息（统一转成字符串后发送）。
pub async fn send_error_message(
    error: anyhow::Error,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    send_copyable_message(error.to_string(), chat_id, client_id).await
}

#[cfg(test)]
mod tests {
    use super::{
        build_message_reply_to, build_target_chat_request_keyboard, build_user_request_keyboard,
    };

    // 结果通知和转存消息位于同一聊天时，必须使用普通回复锚点。
    #[test]
    fn test_build_message_reply_to_uses_same_chat_message() {
        let reply_to = build_message_reply_to(100, 100, 734);

        let tdlib_rs::enums::InputMessageReplyTo::Message(reply) = reply_to else {
            panic!("same-chat result must use an ordinary message reply");
        };
        assert_eq!(reply.message_id, 734);
        assert!(reply.quote.is_none());
        assert_eq!(reply.checklist_task_id, 0);
    }

    // 结果通知和转存消息位于不同聊天时，必须携带目标 chat_id 构造跨聊天回复。
    #[test]
    fn test_build_message_reply_to_uses_external_message() {
        let reply_to = build_message_reply_to(100, -5106953357, 769654784);

        let tdlib_rs::enums::InputMessageReplyTo::ExternalMessage(reply) = reply_to else {
            panic!("cross-chat result must use an external message reply");
        };
        assert_eq!(reply.chat_id, -5106953357);
        assert_eq!(reply.message_id, 769654784);
        assert!(reply.quote.is_none());
        assert_eq!(reply.checklist_task_id, 0);
    }

    #[test]
    fn test_target_chat_request_keyboard_offers_picker_manual_input_and_cancel() {
        let keyboard = build_target_chat_request_keyboard(7001, 7002, "选择目标聊天");

        assert_eq!(keyboard.rows.len(), 3);
        assert_eq!(keyboard.rows[0][0].text, "选择群组");
        assert_eq!(keyboard.rows[0][1].text, "选择频道");
        assert_eq!(keyboard.rows[1][0].text, "手动输入目标");
        assert_eq!(keyboard.rows[2][0].text, "取消");
        assert!(matches!(
            keyboard.rows[1][0].r#type,
            tdlib_rs::enums::KeyboardButtonType::Text
        ));

        let tdlib_rs::enums::KeyboardButtonType::RequestChat(group) = &keyboard.rows[0][0].r#type
        else {
            panic!("group picker must request a chat");
        };
        assert!(!group.chat_is_channel);
        assert!(group.bot_is_member);

        let tdlib_rs::enums::KeyboardButtonType::RequestChat(channel) = &keyboard.rows[0][1].r#type
        else {
            panic!("channel picker must request a chat");
        };
        assert!(channel.chat_is_channel);
        assert!(
            channel
                .bot_administrator_rights
                .as_ref()
                .is_some_and(|rights| rights.can_post_messages)
        );
    }

    #[test]
    fn test_user_request_keyboard_offers_user_picker_and_cancel() {
        let keyboard = build_user_request_keyboard(8101, "选择要授权的用户");

        assert_eq!(keyboard.rows.len(), 2);
        assert_eq!(keyboard.rows[0][0].text, "选择 Telegram 用户");
        assert_eq!(keyboard.rows[1][0].text, "取消");
        let tdlib_rs::enums::KeyboardButtonType::RequestUsers(request) =
            &keyboard.rows[0][0].r#type
        else {
            panic!("user picker must request users");
        };
        assert_eq!(request.id, 8101);
        assert_eq!(request.max_quantity, 1);
        assert!(request.restrict_user_is_bot);
        assert!(!request.user_is_bot);
        assert!(request.request_name);
        assert!(request.request_username);
    }
}
