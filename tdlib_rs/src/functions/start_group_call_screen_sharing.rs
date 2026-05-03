use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Starts screen sharing in a joined group call; not supported in live stories. Returns join response payload for tgcalls
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `audio_source_id` - Screen sharing audio channel synchronization source identifier; received from tgcalls
/// * `payload` - Group call join payload; received from tgcalls
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn start_group_call_screen_sharing(
    group_call_id: i32,
    audio_source_id: i32,
    payload: String,
    client_id: i32,
) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
    "@type": "startGroupCallScreenSharing",
    "group_call_id": group_call_id,
    "audio_source_id": audio_source_id,
    "payload": payload,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
