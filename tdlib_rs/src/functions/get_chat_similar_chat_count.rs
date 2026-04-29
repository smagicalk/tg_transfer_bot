#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns approximate number of chats similar to the given chat
/// # Arguments
/// * `chat_id` - Identifier of the target chat; must be an identifier of a channel chat
/// * `return_local` - Pass true to get the number of chats without sending network requests, or -1 if the number of chats is unknown locally
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_similar_chat_count(chat_id: i64, return_local: bool, client_id: i32) -> Result<crate::enums::Count, crate::types::Error> {
    let request = json!({
        "@type": "getChatSimilarChatCount",
        "chat_id": chat_id,
        "return_local": return_local,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
