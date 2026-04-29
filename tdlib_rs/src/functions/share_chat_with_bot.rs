#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Shares a chat after pressing a keyboardButtonTypeRequestChat button with the bot
/// # Arguments
/// * `chat_id` - Identifier of the chat with the bot
/// * `message_id` - Identifier of the message with the button
/// * `button_id` - Identifier of the button
/// * `shared_chat_id` - Identifier of the shared chat
/// * `only_check` - Pass true to check that the chat can be shared by the button instead of actually sharing it. Doesn't check bot_is_member and bot_administrator_rights restrictions.
    /// If the bot must be a member, then all chats from getGroupsInCommon and all chats, where the user can add the bot, are suitable. In the latter case the bot will be automatically added to the chat.
    /// If the bot must be an administrator, then all chats, where the bot already has requested rights or can be added to administrators by the user, are suitable. In the latter case the bot will be automatically granted requested rights
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn share_chat_with_bot(chat_id: i64, message_id: i64, button_id: i32, shared_chat_id: i64, only_check: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "shareChatWithBot",
        "chat_id": chat_id,
        "message_id": message_id,
        "button_id": button_id,
        "shared_chat_id": shared_chat_id,
        "only_check": only_check,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
