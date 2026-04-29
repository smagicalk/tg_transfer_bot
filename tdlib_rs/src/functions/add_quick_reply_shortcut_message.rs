#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds a message to a quick reply shortcut. If shortcut doesn't exist and there are less than getOption("quick_reply_shortcut_count_max") shortcuts, then a new shortcut is created.
/// The shortcut must not contain more than getOption("quick_reply_shortcut_message_count_max") messages after adding the new message. Returns the added message
/// # Arguments
/// * `shortcut_name` - Name of the target shortcut
/// * `reply_to_message_id` - Identifier of a quick reply message in the same shortcut to be replied; pass 0 if none
/// * `input_message_content` - The content of the message to be added; inputMessagePaidMedia, inputMessageForwarded and inputMessageLocation with live_period aren't supported
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_quick_reply_shortcut_message(shortcut_name: String, reply_to_message_id: i64, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::QuickReplyMessage, crate::types::Error> {
    let request = json!({
        "@type": "addQuickReplyShortcutMessage",
        "shortcut_name": shortcut_name,
        "reply_to_message_id": reply_to_message_id,
        "input_message_content": input_message_content,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
