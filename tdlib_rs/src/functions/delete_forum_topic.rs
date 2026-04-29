#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes all messages from a topic in a forum supergroup chat or a chat with a bot with topics; requires can_delete_messages administrator right in the supergroup
/// unless the user is creator of the topic, the topic has no messages from other users and has at most 11 messages
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `forum_topic_id` - Forum topic identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_forum_topic(chat_id: i64, forum_topic_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteForumTopic",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
