#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds a chat to a chat list. A chat can't be simultaneously in Main and Archive chat lists, so it is automatically removed from another one if needed
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `chat_list` - The chat list. Use getChatListsToAddChat to get suitable chat lists
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_chat_to_list(chat_id: i64, chat_list: crate::enums::ChatList, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addChatToList",
        "chat_id": chat_id,
        "chat_list": chat_list,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
