#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes all messages in the chat. Use chat.can_be_deleted_only_for_self and chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `remove_from_chat_list` - Pass true to remove the chat from all chat lists
/// * `revoke` - Pass true to delete chat history for all users
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_history(chat_id: i64, remove_from_chat_list: bool, revoke: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatHistory",
        "chat_id": chat_id,
        "remove_from_chat_list": remove_from_chat_list,
        "revoke": revoke,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
