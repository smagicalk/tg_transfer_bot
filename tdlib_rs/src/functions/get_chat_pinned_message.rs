#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a newest pinned message in the chat. Returns a 404 error if the message doesn't exist
/// # Arguments
/// * `chat_id` - Identifier of the chat the message belongs to
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_pinned_message(chat_id: i64, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getChatPinnedMessage",
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
