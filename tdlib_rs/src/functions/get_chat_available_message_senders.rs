#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of message sender identifiers, which can be used to send messages in a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_available_message_senders(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatMessageSenders, crate::types::Error> {
    let request = json!({
        "@type": "getChatAvailableMessageSenders",
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
