#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info member right
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `description` - New chat description; 0-255 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_description(chat_id: i64, description: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatDescription",
        "chat_id": chat_id,
        "description": description,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
