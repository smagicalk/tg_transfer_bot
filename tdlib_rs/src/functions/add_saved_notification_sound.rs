#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds a new notification sound to the list of saved notification sounds. The new notification sound is added to the top of the list. If it is already in the list, its position isn't changed
/// # Arguments
/// * `sound` - Notification sound file to add
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_saved_notification_sound(sound: crate::enums::InputFile, client_id: i32) -> Result<crate::enums::NotificationSound, crate::types::Error> {
    let request = json!({
        "@type": "addSavedNotificationSound",
        "sound": sound,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
