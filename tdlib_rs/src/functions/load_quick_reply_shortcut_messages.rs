#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Loads quick reply messages that can be sent by a given quick reply shortcut. The loaded messages will be sent through updateQuickReplyShortcutMessages
/// # Arguments
/// * `shortcut_id` - Unique identifier of the quick reply shortcut
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn load_quick_reply_shortcut_messages(shortcut_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadQuickReplyShortcutMessages",
        "shortcut_id": shortcut_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
