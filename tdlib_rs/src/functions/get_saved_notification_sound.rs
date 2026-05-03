use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns saved notification sound by its identifier. Returns a 404 error if there is no saved notification sound with the specified identifier
/// # Arguments
/// * `notification_sound_id` - Identifier of the notification sound
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_notification_sound(
    notification_sound_id: i64,
    client_id: i32,
) -> Result<crate::enums::NotificationSounds, crate::types::Error> {
    let request = json!({
    "@type": "getSavedNotificationSound",
    "notification_sound_id": notification_sound_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
