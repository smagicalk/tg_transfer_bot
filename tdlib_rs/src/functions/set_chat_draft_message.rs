#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the draft message in a chat or a topic
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `topic_id` - Topic in which the draft will be changed; pass null to change the draft for the chat itself
/// * `draft_message` - New draft message; pass null to remove the draft. All files in draft message content must be of the type inputFileLocal. Media thumbnails and captions are ignored
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_draft_message(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, draft_message: Option<crate::types::DraftMessage>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatDraftMessage",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "draft_message": draft_message,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
