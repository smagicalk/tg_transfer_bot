use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns all scheduled messages in a chat. The messages are returned in reverse chronological order (i.e., in order of decreasing message_id)
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_scheduled_messages(
    chat_id: i64,
    client_id: i32,
) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
    "@type": "getChatScheduledMessages",
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
