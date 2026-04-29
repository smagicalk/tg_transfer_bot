#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message
/// # Arguments
/// * `chat_id` - Identifier of the chat to which to forward messages
/// * `topic_id` - Topic in which the messages will be forwarded; message threads aren't supported; pass null if none
/// * `from_chat_id` - Identifier of the chat from which to forward messages
/// * `message_ids` - Identifiers of the messages to forward. Message identifiers must be in a strictly increasing order. At most 100 messages can be forwarded simultaneously. A message can be forwarded only if messageProperties.can_be_forwarded
/// * `options` - Options to be used to send the messages; pass null to use default options
/// * `send_copy` - Pass true to copy content of the messages without reference to the original sender. Always true if the messages are forwarded to a secret chat or are local.
    /// Use messageProperties.can_be_copied and messageProperties.can_be_copied_to_secret_chat to check whether the message is suitable
/// * `remove_caption` - Pass true to remove media captions of message copies. Ignored if send_copy is false
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn forward_messages(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, from_chat_id: i64, message_ids: Vec<i64>, options: Option<crate::types::MessageSendOptions>, send_copy: bool, remove_caption: bool, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "forwardMessages",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "from_chat_id": from_chat_id,
        "message_ids": message_ids,
        "options": options,
        "send_copy": send_copy,
        "remove_caption": remove_caption,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
