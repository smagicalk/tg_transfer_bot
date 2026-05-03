use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the order of quick reply shortcuts
/// # Arguments
/// * `shortcut_ids` - The new order of quick reply shortcuts
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reorder_quick_reply_shortcuts(
    shortcut_ids: Vec<i32>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "reorderQuickReplyShortcuts",
    "shortcut_ids": shortcut_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
