#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns sparse positions of messages of the specified type in the chat to be used for shared media scroll implementation. Returns the results in reverse chronological order (i.e., in order of decreasing message_id).
/// Cannot be used in secret chats or with searchMessagesFilterFailedToSend filter without an enabled message database
/// # Arguments
/// * `chat_id` - Identifier of the chat in which to return information about message positions
/// * `filter` - Filter for message content. Filters searchMessagesFilterEmpty, searchMessagesFilterMention, searchMessagesFilterUnreadMention, and searchMessagesFilterUnreadReaction are unsupported in this function
/// * `from_message_id` - The message identifier from which to return information about message positions
/// * `limit` - The expected number of message positions to be returned; 50-2000. A smaller number of positions can be returned, if there are not enough appropriate messages
/// * `saved_messages_topic_id` - If not 0, only messages in the specified Saved Messages topic will be considered; pass 0 to consider all messages, or for chats other than Saved Messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_sparse_message_positions(chat_id: i64, filter: crate::enums::SearchMessagesFilter, from_message_id: i64, limit: i32, saved_messages_topic_id: i64, client_id: i32) -> Result<crate::enums::MessagePositions, crate::types::Error> {
    let request = json!({
        "@type": "getChatSparseMessagePositions",
        "chat_id": chat_id,
        "filter": filter,
        "from_message_id": from_message_id,
        "limit": limit,
        "saved_messages_topic_id": saved_messages_topic_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
