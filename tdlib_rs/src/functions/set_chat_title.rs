use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the chat title. Supported only for basic groups, supergroups and channels. Requires can_change_info member right
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `title` - New title of the chat; 1-128 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_title(
    chat_id: i64,
    title: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setChatTitle",
    "chat_id": chat_id,
    "title": title,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
