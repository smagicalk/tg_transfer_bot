#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes all messages between the specified dates in a chat. Supported only for private chats and basic groups. Messages sent in the last 30 seconds will not be deleted
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `min_date` - The minimum date of the messages to delete
/// * `max_date` - The maximum date of the messages to delete
/// * `revoke` - Pass true to delete chat messages for all users; private chats only
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_messages_by_date(chat_id: i64, min_date: i32, max_date: i32, revoke: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatMessagesByDate",
        "chat_id": chat_id,
        "min_date": min_date,
        "max_date": max_date,
        "revoke": revoke,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
