use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Traverses all chats in a chat list and marks all messages in the chats as read
/// # Arguments
/// * `chat_list` - Chat list in which to mark all chats as read
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn read_chat_list(
    chat_list: crate::enums::ChatList,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "readChatList",
    "chat_list": chat_list,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
