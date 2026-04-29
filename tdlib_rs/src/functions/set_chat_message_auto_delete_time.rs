#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the message auto-delete or self-destruct (for secret chats) time in a chat. Requires change_info administrator right in basic groups, supergroups and channels.
/// Message auto-delete time can't be changed in a chat with the current user (Saved Messages) and the chat 777000 (Telegram).
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `message_auto_delete_time` - New time value, in seconds; unless the chat is secret, it must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_message_auto_delete_time(chat_id: i64, message_auto_delete_time: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatMessageAutoDeleteTime",
        "chat_id": chat_id,
        "message_auto_delete_time": message_auto_delete_time,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
