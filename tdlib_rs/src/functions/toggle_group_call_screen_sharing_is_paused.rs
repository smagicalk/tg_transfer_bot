#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Pauses or unpauses screen sharing in a joined group call; not supported in live stories
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `is_paused` - Pass true to pause screen sharing; pass false to unpause it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_group_call_screen_sharing_is_paused(group_call_id: i32, is_paused: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleGroupCallScreenSharingIsPaused",
        "group_call_id": group_call_id,
        "is_paused": is_paused,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
