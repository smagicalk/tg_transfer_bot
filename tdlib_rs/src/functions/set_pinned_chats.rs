use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the order of pinned chats
/// # Arguments
/// * `chat_list` - Chat list in which to change the order of pinned chats
/// * `chat_ids` - The new list of pinned chats
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_pinned_chats(
    chat_list: crate::enums::ChatList,
    chat_ids: Vec<i64>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setPinnedChats",
    "chat_list": chat_list,
    "chat_ids": chat_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
