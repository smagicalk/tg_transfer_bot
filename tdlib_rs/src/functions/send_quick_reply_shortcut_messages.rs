use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends messages from a quick reply shortcut. Requires Telegram Business subscription. Can't be used to send paid messages
/// # Arguments
/// * `chat_id` - Identifier of the chat to which to send messages. The chat must be a private chat with a regular user
/// * `shortcut_id` - Unique identifier of the quick reply shortcut
/// * `sending_id` - Non-persistent identifier, which will be returned back in messageSendingStatePending object and can be used to match sent messages and corresponding updateNewMessage updates
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_quick_reply_shortcut_messages(
    chat_id: i64,
    shortcut_id: i32,
    sending_id: i32,
    client_id: i32,
) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
    "@type": "sendQuickReplyShortcutMessages",
    "chat_id": chat_id,
    "shortcut_id": shortcut_id,
    "sending_id": sending_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
