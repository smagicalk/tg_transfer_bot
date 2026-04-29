#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends a notification about user activity in a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `topic_id` - Identifier of the topic in which the action is performed; pass null if none
/// * `action` - The action description; pass null to cancel the currently active action
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_chat_action(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, action: Option<crate::enums::ChatAction>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendChatAction",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "action": action,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
