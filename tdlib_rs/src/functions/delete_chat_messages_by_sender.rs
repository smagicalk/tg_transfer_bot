#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes all messages sent by the specified message sender in a chat. Supported only for supergroups; requires can_delete_messages administrator right
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `sender_id` - Identifier of the sender of messages to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_messages_by_sender(chat_id: i64, sender_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatMessagesBySender",
        "chat_id": chat_id,
        "sender_id": sender_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
