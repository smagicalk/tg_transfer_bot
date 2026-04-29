#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Checks whether a username can be set for a chat
/// # Arguments
/// * `chat_id` - Chat identifier; must be identifier of a supergroup chat, or a channel chat, or a private chat with self, or 0 if the chat is being created
/// * `username` - Username to be checked
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_chat_username(chat_id: i64, username: String, client_id: i32) -> Result<crate::enums::CheckChatUsernameResult, crate::types::Error> {
    let request = json!({
        "@type": "checkChatUsername",
        "chat_id": chat_id,
        "username": username,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
