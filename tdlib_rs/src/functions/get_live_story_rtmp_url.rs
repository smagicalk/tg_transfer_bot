use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns RTMP URL for streaming to a live story; requires can_post_stories administrator right for channel chats
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_live_story_rtmp_url(
    chat_id: i64,
    client_id: i32,
) -> Result<crate::enums::RtmpUrl, crate::types::Error> {
    let request = json!({
    "@type": "getLiveStoryRtmpUrl",
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
