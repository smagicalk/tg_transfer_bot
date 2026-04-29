#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the value of the default disable_notification parameter, used when a message is sent to a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `default_disable_notification` - New value of default_disable_notification
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_default_disable_notification(chat_id: i64, default_disable_notification: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatDefaultDisableNotification",
        "chat_id": chat_id,
        "default_disable_notification": default_disable_notification,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
