#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns an HTML code for embedding the message. Available only if messageProperties.can_get_embedding_code
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `for_album` - Pass true to return an HTML code for embedding of the whole media album
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_embedding_code(chat_id: i64, message_id: i64, for_album: bool, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getMessageEmbeddingCode",
        "chat_id": chat_id,
        "message_id": message_id,
        "for_album": for_album,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
