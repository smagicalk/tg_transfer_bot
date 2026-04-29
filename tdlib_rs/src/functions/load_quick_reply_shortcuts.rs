#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Loads quick reply shortcuts created by the current user. The loaded data will be sent through updateQuickReplyShortcut and updateQuickReplyShortcuts
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn load_quick_reply_shortcuts(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadQuickReplyShortcuts",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
