#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Marks all mentions in a topic in a forum supergroup chat as read
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - Forum topic identifier in which mentions are marked as read
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn read_all_forum_topic_mentions(chat_id: i64, forum_topic_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "readAllForumTopicMentions",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
