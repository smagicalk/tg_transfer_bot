#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns messages in a topic in a forum supergroup chat or a chat with a bot with topics. The messages are returned in reverse chronological order
/// (i.e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - Forum topic identifier
/// * `from_message_id` - Identifier of the message starting from which history must be fetched; use 0 to get results from the last message
/// * `offset` - Specify 0 to get results from exactly the message from_message_id or a negative number from -99 to -1 to get additionally -offset newer messages
/// * `limit` - The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, then the limit must be greater than or equal to -offset.
    /// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_forum_topic_history(chat_id: i64, forum_topic_id: i32, from_message_id: i64, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "getForumTopicHistory",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        "from_message_id": from_message_id,
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
