use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns an ordered list of chats from the beginning of a chat list. For informational purposes only. Use loadChats and updates processing instead to maintain chat lists in a consistent state
/// # Arguments
/// * `chat_list` - The chat list in which to return chats; pass null to get chats from the main chat list
/// * `limit` - The maximum number of chats to be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chats(
    chat_list: Option<crate::enums::ChatList>,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
    "@type": "getChats",
    "chat_list": chat_list,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
