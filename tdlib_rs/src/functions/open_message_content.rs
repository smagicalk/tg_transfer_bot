use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message).
/// An updateMessageContentOpened update will be generated if something has changed
/// # Arguments
/// * `chat_id` - Chat identifier of the message
/// * `message_id` - Identifier of the message with the opened content
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn open_message_content(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "openMessageContent",
    "chat_id": chat_id,
    "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
