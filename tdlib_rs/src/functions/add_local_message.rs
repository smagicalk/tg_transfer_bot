#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message
/// # Arguments
/// * `chat_id` - Target chat; channel direct messages chats aren't supported
/// * `sender_id` - Identifier of the sender of the message
/// * `reply_to` - Information about the message or story to be replied; pass null if none
/// * `disable_notification` - Pass true to disable notification for the message
/// * `input_message_content` - The content of the message to be added
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_local_message(chat_id: i64, sender_id: crate::enums::MessageSender, reply_to: Option<crate::enums::InputMessageReplyTo>, disable_notification: bool, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "addLocalMessage",
        "chat_id": chat_id,
        "sender_id": sender_id,
        "reply_to": reply_to,
        "disable_notification": disable_notification,
        "input_message_content": input_message_content,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
