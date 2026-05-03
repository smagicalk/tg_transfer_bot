use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether a topic is closed in a forum supergroup chat; requires can_manage_topics administrator right in the supergroup unless the user is creator of the topic
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `forum_topic_id` - Forum topic identifier
/// * `is_closed` - Pass true to close the topic; pass false to reopen it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_forum_topic_is_closed(
    chat_id: i64,
    forum_topic_id: i32,
    is_closed: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleForumTopicIsClosed",
    "chat_id": chat_id,
    "forum_topic_id": forum_topic_id,
    "is_closed": is_closed,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
