#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about the next messages of the specified type in the chat split by days. Returns the results in reverse chronological order. Can return partial result for the last returned day. Behavior of this method depends on the value of the option "utc_time_offset"
/// # Arguments
/// * `chat_id` - Identifier of the chat in which to return information about messages
/// * `topic_id` - Pass topic identifier to get the result only in specific topic; pass null to get the result in all topics; forum topics and message threads aren't supported
/// * `filter` - Filter for message content. Filters searchMessagesFilterEmpty, searchMessagesFilterMention, searchMessagesFilterUnreadMention, and searchMessagesFilterUnreadReaction are unsupported in this function
/// * `from_message_id` - The message identifier from which to return information about messages; use 0 to get results from the last message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_message_calendar(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, filter: crate::enums::SearchMessagesFilter, from_message_id: i64, client_id: i32) -> Result<crate::enums::MessageCalendar, crate::types::Error> {
    let request = json!({
        "@type": "getChatMessageCalendar",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "filter": filter,
        "from_message_id": from_message_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
