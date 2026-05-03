use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user
/// # Arguments
/// * `notification_group_id` - Identifier of notification group to which the notification belongs
/// * `notification_id` - Identifier of removed notification
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_notification(
    notification_group_id: i32,
    notification_id: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "removeNotification",
    "notification_group_id": notification_group_id,
    "notification_id": notification_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
