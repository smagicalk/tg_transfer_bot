use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes notification settings for chats of a given type
/// # Arguments
/// * `scope` - Types of chats for which to change the notification settings
/// * `notification_settings` - The new notification settings for the given scope
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_scope_notification_settings(
    scope: crate::enums::NotificationSettingsScope,
    notification_settings: crate::types::ScopeNotificationSettings,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setScopeNotificationSettings",
    "scope": scope,
    "notification_settings": notification_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
