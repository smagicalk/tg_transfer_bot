use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of chats with non-default notification settings for new messages
/// # Arguments
/// * `scope` - If specified, only chats from the scope will be returned; pass null to return chats from all scopes
/// * `compare_sound` - Pass true to include in the response chats with only non-default sound
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_notification_settings_exceptions(
    scope: Option<crate::enums::NotificationSettingsScope>,
    compare_sound: bool,
    client_id: i32,
) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
    "@type": "getChatNotificationSettingsExceptions",
    "scope": scope,
    "compare_sound": compare_sound,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
