#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the pinned state of a topic in a forum supergroup chat or a chat with a bot with topics; requires can_manage_topics administrator right in the supergroup.
/// There can be up to getOption("pinned_forum_topic_count_max") pinned forum topics
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - Forum topic identifier
/// * `is_pinned` - Pass true to pin the topic; pass false to unpin it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_forum_topic_is_pinned(chat_id: i64, forum_topic_id: i32, is_pinned: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleForumTopicIsPinned",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        "is_pinned": is_pinned,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
