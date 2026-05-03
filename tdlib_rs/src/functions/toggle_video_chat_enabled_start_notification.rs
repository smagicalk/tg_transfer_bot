use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether the current user will receive a notification when the video chat starts; for scheduled video chats only
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `enabled_start_notification` - New value of the enabled_start_notification setting
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_video_chat_enabled_start_notification(
    group_call_id: i32,
    enabled_start_notification: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleVideoChatEnabledStartNotification",
    "group_call_id": group_call_id,
    "enabled_start_notification": enabled_start_notification,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
