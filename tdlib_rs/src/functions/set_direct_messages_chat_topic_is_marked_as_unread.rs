#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the marked as unread state of the topic in a channel direct messages chat administered by the current user
/// # Arguments
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `topic_id` - Topic identifier
/// * `is_marked_as_unread` - New value of is_marked_as_unread
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_direct_messages_chat_topic_is_marked_as_unread(chat_id: i64, topic_id: i64, is_marked_as_unread: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setDirectMessagesChatTopicIsMarkedAsUnread",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "is_marked_as_unread": is_marked_as_unread,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
