use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Pins a message in a chat. A message can be pinned only if messageProperties.can_be_pinned
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `message_id` - Identifier of the new pinned message
/// * `disable_notification` - Pass true to disable notification about the pinned message. Notifications are always disabled in channels and private chats
/// * `only_for_self` - Pass true to pin the message only for self; private chats only
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn pin_chat_message(
    chat_id: i64,
    message_id: i64,
    disable_notification: bool,
    only_for_self: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "pinChatMessage",
    "chat_id": chat_id,
    "message_id": message_id,
    "disable_notification": disable_notification,
    "only_for_self": only_for_self,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
