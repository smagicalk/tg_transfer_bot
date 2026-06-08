use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a list of chats similar to the given chat
/// # Arguments
/// * `chat_id` - Identifier of the target chat; must be an identifier of a channel chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_similar_chats(
    chat_id: i64,
    client_id: i32,
) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
    "@type": "getChatSimilarChats",
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
