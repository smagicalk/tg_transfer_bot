use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Starts a scheduled video chat
/// # Arguments
/// * `group_call_id` - Group call identifier of the video chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn start_scheduled_video_chat(
    group_call_id: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "startScheduledVideoChat",
    "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
