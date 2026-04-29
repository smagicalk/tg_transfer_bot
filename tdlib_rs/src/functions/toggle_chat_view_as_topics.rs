#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the view_as_topics setting of a forum chat or Saved Messages
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `view_as_topics` - New value of view_as_topics
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_view_as_topics(chat_id: i64, view_as_topics: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatViewAsTopics",
        "chat_id": chat_id,
        "view_as_topics": view_as_topics,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
