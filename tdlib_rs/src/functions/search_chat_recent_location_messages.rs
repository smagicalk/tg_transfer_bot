use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about the recent locations of chat members that were sent to the chat. Returns up to 1 location message per user
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `limit` - The maximum number of messages to be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_chat_recent_location_messages(
    chat_id: i64,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
    "@type": "searchChatRecentLocationMessages",
    "chat_id": chat_id,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
