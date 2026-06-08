use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Marks all reactions in a topic in a forum supergroup chat or a chat with a bot with topics as read
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - Forum topic identifier in which reactions are marked as read
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn read_all_forum_topic_reactions(
    chat_id: i64,
    forum_topic_id: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "readAllForumTopicReactions",
    "chat_id": chat_id,
    "forum_topic_id": forum_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
