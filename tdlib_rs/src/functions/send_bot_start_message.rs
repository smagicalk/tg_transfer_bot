use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Invites a bot to a chat (if it is not yet a member) and sends it the /start command; requires can_invite_users member right. Bots can't be invited to a private chat other than the chat with the bot.
/// Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message
/// # Arguments
/// * `bot_user_id` - Identifier of the bot
/// * `chat_id` - Identifier of the target chat
/// * `parameter` - A hidden parameter sent to the bot for deep linking purposes (https:core.telegram.org/bots#deep-linking)
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_bot_start_message(
    bot_user_id: i64,
    chat_id: i64,
    parameter: String,
    client_id: i32,
) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
    "@type": "sendBotStartMessage",
    "bot_user_id": bot_user_id,
    "chat_id": chat_id,
    "parameter": parameter,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
