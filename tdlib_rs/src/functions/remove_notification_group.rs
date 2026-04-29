#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user
/// # Arguments
/// * `notification_group_id` - Notification group identifier
/// * `max_notification_id` - The maximum identifier of removed notifications
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_notification_group(notification_group_id: i32, max_notification_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeNotificationGroup",
        "notification_group_id": notification_group_id,
        "max_notification_id": max_notification_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
