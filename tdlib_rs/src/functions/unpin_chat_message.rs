use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes a pinned message from a chat; requires can_pin_messages member right if the chat is a basic group or supergroup, or can_edit_messages administrator right if the chat is a channel
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `message_id` - Identifier of the removed pinned message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn unpin_chat_message(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "unpinChatMessage",
    "chat_id": chat_id,
    "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
