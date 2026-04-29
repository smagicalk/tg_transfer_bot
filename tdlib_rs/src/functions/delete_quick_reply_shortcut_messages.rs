#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes specified quick reply messages
/// # Arguments
/// * `shortcut_id` - Unique identifier of the quick reply shortcut to which the messages belong
/// * `message_ids` - Unique identifiers of the messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_quick_reply_shortcut_messages(shortcut_id: i32, message_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteQuickReplyShortcutMessages",
        "shortcut_id": shortcut_id,
        "message_ids": message_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
