#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query
/// (searchSecretMessages must be used instead), or without an enabled message database. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit.
/// A combination of query, sender_id, filter and topic_id search criteria is expected to be supported, only if it is required for Telegram official application implementation
/// # Arguments
/// * `chat_id` - Identifier of the chat in which to search messages
/// * `topic_id` - Pass topic identifier to search messages only in specific topic; pass null to search for messages in all topics
/// * `query` - Query to search for
/// * `sender_id` - Identifier of the sender of messages to search for; pass null to search for messages from any sender. Not supported in secret chats
/// * `from_message_id` - Identifier of the message starting from which history must be fetched; use 0 to get results from the last message
/// * `offset` - Specify 0 to get results from exactly the message from_message_id or a negative number to get the specified message and some newer messages
/// * `limit` - The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, then the limit must be greater than -offset.
    /// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `filter` - Additional filter for messages to search; pass null to search for all messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_chat_messages(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, query: String, sender_id: Option<crate::enums::MessageSender>, from_message_id: i64, offset: i32, limit: i32, filter: Option<crate::enums::SearchMessagesFilter>, client_id: i32) -> Result<crate::enums::FoundChatMessages, crate::types::Error> {
    let request = json!({
        "@type": "searchChatMessages",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "query": query,
        "sender_id": sender_id,
        "from_message_id": from_message_id,
        "offset": offset,
        "limit": limit,
        "filter": filter,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
