#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Loads more topics in a channel direct messages chat administered by the current user. The loaded topics will be sent through updateDirectMessagesChatTopic.
/// Topics are sorted by their topic.order in descending order. Returns a 404 error if all topics have been loaded
/// # Arguments
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `limit` - The maximum number of topics to be loaded. For optimal performance, the number of loaded topics is chosen by TDLib and can be smaller than the specified limit, even if the end of the list is not reached
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn load_direct_messages_chat_topics(chat_id: i64, limit: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadDirectMessagesChatTopics",
        "chat_id": chat_id,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
