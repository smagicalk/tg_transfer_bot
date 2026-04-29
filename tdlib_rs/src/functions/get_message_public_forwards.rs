#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns forwarded copies of a channel message to different public channels and public reposts as a story. Can be used only if messageProperties.can_get_statistics == true. For optimal performance, the number of returned messages and stories is chosen by TDLib
/// # Arguments
/// * `chat_id` - Chat identifier of the message
/// * `message_id` - Message identifier
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages and stories to be returned; must be positive and can't be greater than 100. For optimal performance, the number of returned objects is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_public_forwards(chat_id: i64, message_id: i64, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::PublicForwards, crate::types::Error> {
    let request = json!({
        "@type": "getMessagePublicForwards",
        "chat_id": chat_id,
        "message_id": message_id,
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
