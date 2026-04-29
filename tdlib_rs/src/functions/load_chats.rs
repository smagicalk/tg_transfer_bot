#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Loads more chats from a chat list. The loaded chats and their positions in the chat list will be sent through updates. Chats are sorted by the pair (chat.position.order, chat.id) in descending order. Returns a 404 error if all chats have been loaded
/// # Arguments
/// * `chat_list` - The chat list in which to load chats; pass null to load chats from the main chat list
/// * `limit` - The maximum number of chats to be loaded. For optimal performance, the number of loaded chats is chosen by TDLib and can be smaller than the specified limit, even if the end of the list is not reached
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn load_chats(chat_list: Option<crate::enums::ChatList>, limit: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadChats",
        "chat_list": chat_list,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
