use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns found forum topics in a forum supergroup chat or a chat with a bot with topics. This is a temporary method for getting information about topic list from the server
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `query` - Query to search for in the forum topic's name
/// * `offset_date` - The date starting from which the results need to be fetched. Use 0 or any date in the future to get results from the last topic
/// * `offset_message_id` - The message identifier of the last message in the last found topic, or 0 for the first request
/// * `offset_forum_topic_id` - The forum topic identifier of the last found topic, or 0 for the first request
/// * `limit` - The maximum number of forum topics to be returned; up to 100. For optimal performance, the number of returned forum topics is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_forum_topics(
    chat_id: i64,
    query: String,
    offset_date: i32,
    offset_message_id: i64,
    offset_forum_topic_id: i32,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::ForumTopics, crate::types::Error> {
    let request = json!({
    "@type": "getForumTopics",
    "chat_id": chat_id,
    "query": query,
    "offset_date": offset_date,
    "offset_message_id": offset_message_id,
    "offset_forum_topic_id": offset_forum_topic_id,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
