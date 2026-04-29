#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a message. Returns a 404 error if the message doesn't exist
/// # Arguments
/// * `chat_id` - Identifier of the chat the message belongs to
/// * `message_id` - Identifier of the message to get
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getMessage",
        "chat_id": chat_id,
        "message_id": message_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
