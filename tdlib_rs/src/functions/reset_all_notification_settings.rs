use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Resets all chat and scope notification settings to their default values. By default, all chats are unmuted and message previews are shown
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reset_all_notification_settings(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "resetAllNotificationSettings",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
