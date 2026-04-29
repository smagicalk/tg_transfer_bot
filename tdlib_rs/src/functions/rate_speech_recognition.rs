#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Rates recognized speech in a video note or a voice note message
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `is_good` - Pass true if the speech recognition is good
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn rate_speech_recognition(chat_id: i64, message_id: i64, is_good: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "rateSpeechRecognition",
        "chat_id": chat_id,
        "message_id": message_id,
        "is_good": is_good,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
