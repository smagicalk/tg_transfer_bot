use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of saved notification sounds. If a sound isn't in the list, then default sound needs to be used
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_notification_sounds(
    client_id: i32,
) -> Result<crate::enums::NotificationSounds, crate::types::Error> {
    let request = json!({
    "@type": "getSavedNotificationSounds",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
