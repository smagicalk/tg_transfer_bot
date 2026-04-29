#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes name of a quick reply shortcut
/// # Arguments
/// * `shortcut_id` - Unique identifier of the quick reply shortcut
/// * `name` - New name for the shortcut. Use checkQuickReplyShortcutName to check its validness
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_quick_reply_shortcut_name(shortcut_id: i32, name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setQuickReplyShortcutName",
        "shortcut_id": shortcut_id,
        "name": name,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
