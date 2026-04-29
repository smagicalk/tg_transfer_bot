#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about actual author of a message sent on behalf of a channel. The method can be called if messageProperties.can_get_author == true
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `message_id` - Identifier of the message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_author(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
        "@type": "getMessageAuthor",
        "chat_id": chat_id,
        "message_id": message_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
