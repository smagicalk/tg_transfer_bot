#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns approximate 1-based position of a message among messages, which can be found by the specified filter in the chat and topic. Cannot be used in secret chats
/// # Arguments
/// * `chat_id` - Identifier of the chat in which to find message position
/// * `topic_id` - Pass topic identifier to get position among messages only in specific topic; pass null to get position among all chat messages; message threads aren't supported
/// * `filter` - Filter for message content; searchMessagesFilterEmpty, searchMessagesFilterUnreadMention, searchMessagesFilterUnreadReaction, and searchMessagesFilterFailedToSend are unsupported in this function
/// * `message_id` - Message identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_message_position(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, filter: crate::enums::SearchMessagesFilter, message_id: i64, client_id: i32) -> Result<crate::enums::Count, crate::types::Error> {
    let request = json!({
        "@type": "getChatMessagePosition",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "filter": filter,
        "message_id": message_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
