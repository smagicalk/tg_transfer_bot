use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns sponsored messages to be shown in a chat; for channel chats and chats with bots only
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_sponsored_messages(
    chat_id: i64,
    client_id: i32,
) -> Result<crate::enums::SponsoredMessages, crate::types::Error> {
    let request = json!({
    "@type": "getChatSponsoredMessages",
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
