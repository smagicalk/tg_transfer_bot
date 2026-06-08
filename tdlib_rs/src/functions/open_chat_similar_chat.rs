use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Informs TDLib that a chat was opened from the list of similar chats. The method is independent of openChat and closeChat methods
/// # Arguments
/// * `chat_id` - Identifier of the original chat, which similar chats were requested
/// * `opened_chat_id` - Identifier of the opened chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn open_chat_similar_chat(
    chat_id: i64,
    opened_chat_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "openChatSimilarChat",
    "chat_id": chat_id,
    "opened_chat_id": opened_chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
