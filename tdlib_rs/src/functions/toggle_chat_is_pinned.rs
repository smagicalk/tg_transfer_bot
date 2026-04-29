#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the pinned state of a chat. There can be up to getOption("pinned_chat_count_max")/getOption("pinned_archived_chat_count_max") pinned non-secret chats and the same number of secret chats in the main/archive chat list. The limit can be increased with Telegram Premium
/// # Arguments
/// * `chat_list` - Chat list in which to change the pinned state of the chat
/// * `chat_id` - Chat identifier
/// * `is_pinned` - Pass true to pin the chat; pass false to unpin it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_is_pinned(chat_list: crate::enums::ChatList, chat_id: i64, is_pinned: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatIsPinned",
        "chat_list": chat_list,
        "chat_id": chat_id,
        "is_pinned": is_pinned,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
