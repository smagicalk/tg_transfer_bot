#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Checks validness of a name for a quick reply shortcut. Can be called synchronously
/// # Arguments
/// * `name` - The name of the shortcut; 1-32 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_quick_reply_shortcut_name(name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkQuickReplyShortcutName",
        "name": name,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
