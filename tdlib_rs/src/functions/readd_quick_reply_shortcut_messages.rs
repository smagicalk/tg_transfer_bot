use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Readds quick reply messages which failed to add. Can be called only for messages for which messageSendingStateFailed.can_retry is true and after specified in messageSendingStateFailed.retry_after time passed.
/// If a message is readded, the corresponding failed to send message is deleted. Returns the sent messages in the same order as the message identifiers passed in message_ids. If a message can't be readded, null will be returned instead of the message
/// # Arguments
/// * `shortcut_name` - Name of the target shortcut
/// * `message_ids` - Identifiers of the quick reply messages to readd. Message identifiers must be in a strictly increasing order
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn readd_quick_reply_shortcut_messages(
    shortcut_name: String,
    message_ids: Vec<i64>,
    client_id: i32,
) -> Result<crate::enums::QuickReplyMessages, crate::types::Error> {
    let request = json!({
    "@type": "readdQuickReplyShortcutMessages",
    "shortcut_name": shortcut_name,
    "message_ids": message_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
