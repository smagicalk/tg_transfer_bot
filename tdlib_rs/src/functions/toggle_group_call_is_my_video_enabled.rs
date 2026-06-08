use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether current user's video is enabled
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `is_my_video_enabled` - Pass true if the current user's video is enabled
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_group_call_is_my_video_enabled(
    group_call_id: i32,
    is_my_video_enabled: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleGroupCallIsMyVideoEnabled",
    "group_call_id": group_call_id,
    "is_my_video_enabled": is_my_video_enabled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
