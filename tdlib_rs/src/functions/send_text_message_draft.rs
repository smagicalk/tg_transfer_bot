use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends a draft for a being generated text message; for bots only
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - The forum topic identifier in which the message will be sent; pass 0 if none
/// * `draft_id` - Unique identifier of the draft
/// * `text` - Draft text of the message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_text_message_draft(
    chat_id: i64,
    forum_topic_id: i32,
    draft_id: i64,
    text: crate::types::FormattedText,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "sendTextMessageDraft",
    "chat_id": chat_id,
    "forum_topic_id": forum_topic_id,
    "draft_id": draft_id,
    "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
