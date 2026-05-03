use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about the user or the chat that streams to a live story; for live stories that aren't an RTMP stream only
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_live_story_streamer(
    group_call_id: i32,
    client_id: i32,
) -> Result<crate::enums::GroupCallParticipant, crate::types::Error> {
    let request = json!({
    "@type": "getLiveStoryStreamer",
    "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
