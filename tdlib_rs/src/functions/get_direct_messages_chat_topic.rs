#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about the topic in a channel direct messages chat administered by the current user
/// # Arguments
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `topic_id` - Identifier of the topic to get
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_direct_messages_chat_topic(chat_id: i64, topic_id: i64, client_id: i32) -> Result<crate::enums::DirectMessagesChatTopic, crate::types::Error> {
    let request = json!({
        "@type": "getDirectMessagesChatTopic",
        "chat_id": chat_id,
        "topic_id": topic_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
