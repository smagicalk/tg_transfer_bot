use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the order of pinned topics in a forum supergroup chat or a chat with a bot with topics; requires can_manage_topics administrator right in the supergroup
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `forum_topic_ids` - The new list of identifiers of the pinned forum topics
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_pinned_forum_topics(
    chat_id: i64,
    forum_topic_ids: Vec<i32>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setPinnedForumTopics",
    "chat_id": chat_id,
    "forum_topic_ids": forum_topic_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
