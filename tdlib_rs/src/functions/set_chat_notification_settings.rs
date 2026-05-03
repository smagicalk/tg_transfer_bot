use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the notification settings of a chat. Notification settings of a chat with the current user (Saved Messages) can't be changed
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `notification_settings` - New notification settings for the chat. If the chat is muted for more than 366 days, it is considered to be muted forever
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_notification_settings(
    chat_id: i64,
    notification_settings: crate::types::ChatNotificationSettings,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setChatNotificationSettings",
    "chat_id": chat_id,
    "notification_settings": notification_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
