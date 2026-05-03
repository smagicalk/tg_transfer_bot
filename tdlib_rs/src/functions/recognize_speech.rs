use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Recognizes speech in a video note or a voice note message
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message. Use messageProperties.can_recognize_speech to check whether the message is suitable
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn recognize_speech(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "recognizeSpeech",
    "chat_id": chat_id,
    "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
