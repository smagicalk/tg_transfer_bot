use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether notifications for new gifts received by a channel chat are sent to the current user; requires can_post_messages administrator right in the chat
/// # Arguments
/// * `chat_id` - Identifier of the channel chat
/// * `are_enabled` - Pass true to enable notifications about new gifts owned by the channel chat; pass false to disable the notifications
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_gift_notifications(
    chat_id: i64,
    are_enabled: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleChatGiftNotifications",
    "chat_id": chat_id,
    "are_enabled": are_enabled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
