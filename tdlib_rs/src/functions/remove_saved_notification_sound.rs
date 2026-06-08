use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes a notification sound from the list of saved notification sounds
/// # Arguments
/// * `notification_sound_id` - Identifier of the notification sound
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_saved_notification_sound(
    notification_sound_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "removeSavedNotificationSound",
    "notification_sound_id": notification_sound_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
