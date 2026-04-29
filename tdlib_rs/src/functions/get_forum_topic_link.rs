#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns an HTTPS link to a topic in a forum supergroup chat. This is an offline method
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `forum_topic_id` - Forum topic identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_forum_topic_link(chat_id: i64, forum_topic_id: i32, client_id: i32) -> Result<crate::enums::MessageLink, crate::types::Error> {
    let request = json!({
        "@type": "getForumTopicLink",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
