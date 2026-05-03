use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends a message. Returns the sent message
/// # Arguments
/// * `chat_id` - Target chat
/// * `topic_id` - Topic in which the message will be sent; pass null if none
/// * `reply_to` - Information about the message or story to be replied; pass null if none
/// * `options` - Options to be used to send the message; pass null to use default options
/// * `input_message_content` - The content of the message to be sent
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_message(
    chat_id: i64,
    topic_id: Option<crate::enums::MessageTopic>,
    reply_to: Option<crate::enums::InputMessageReplyTo>,
    options: Option<crate::types::MessageSendOptions>,
    input_message_content: crate::enums::InputMessageContent,
    client_id: i32,
) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
    "@type": "sendMessage",
    "chat_id": chat_id,
    "topic_id": topic_id,
    "reply_to": reply_to,
    "options": options,
    "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
