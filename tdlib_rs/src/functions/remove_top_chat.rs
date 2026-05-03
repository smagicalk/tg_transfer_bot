use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled
/// # Arguments
/// * `category` - Category of frequently used chats
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_top_chat(
    category: crate::enums::TopChatCategory,
    chat_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "removeTopChat",
    "category": category,
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
