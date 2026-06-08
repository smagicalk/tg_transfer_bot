use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns advertisements to be shown while a video from a message is watched. Available only if messageProperties.can_get_video_advertisements
/// # Arguments
/// * `chat_id` - Identifier of the chat with the message
/// * `message_id` - Identifier of the message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_video_message_advertisements(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> Result<crate::enums::VideoMessageAdvertisements, crate::types::Error> {
    let request = json!({
    "@type": "getVideoMessageAdvertisements",
    "chat_id": chat_id,
    "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
