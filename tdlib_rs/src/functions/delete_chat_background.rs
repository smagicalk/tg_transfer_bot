#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes background in a specific chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `restore_previous` - Pass true to restore previously set background. Can be used only in private and secret chats with non-deleted users if userFullInfo.set_chat_background == true.
    /// Supposed to be used from messageChatSetBackground messages with the currently set background that was set for both sides by the other user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_background(chat_id: i64, restore_previous: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatBackground",
        "chat_id": chat_id,
        "restore_previous": restore_previous,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
