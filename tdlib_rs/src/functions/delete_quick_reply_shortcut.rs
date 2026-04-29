#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes a quick reply shortcut
/// # Arguments
/// * `shortcut_id` - Unique identifier of the quick reply shortcut
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_quick_reply_shortcut(shortcut_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteQuickReplyShortcut",
        "shortcut_id": shortcut_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
