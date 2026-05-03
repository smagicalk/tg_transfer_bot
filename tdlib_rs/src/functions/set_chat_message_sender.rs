use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Selects a message sender to send messages in a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `message_sender_id` - New message sender for the chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_message_sender(
    chat_id: i64,
    message_sender_id: crate::enums::MessageSender,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setChatMessageSender",
    "chat_id": chat_id,
    "message_sender_id": message_sender_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
