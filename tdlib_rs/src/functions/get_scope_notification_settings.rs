#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the notification settings for chats of a given type
/// # Arguments
/// * `scope` - Types of chats for which to return the notification settings information
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_scope_notification_settings(scope: crate::enums::NotificationSettingsScope, client_id: i32) -> Result<crate::enums::ScopeNotificationSettings, crate::types::Error> {
    let request = json!({
        "@type": "getScopeNotificationSettings",
        "scope": scope,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
