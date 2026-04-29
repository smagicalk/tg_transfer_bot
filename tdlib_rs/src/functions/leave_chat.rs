#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes the current user from chat members. Private and secret chats can't be left using this method
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn leave_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "leaveChat",
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
