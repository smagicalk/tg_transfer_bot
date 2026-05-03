use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether chat folder tags are enabled
/// # Arguments
/// * `are_tags_enabled` - Pass true to enable folder tags; pass false to disable them
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_folder_tags(
    are_tags_enabled: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleChatFolderTags",
    "are_tags_enabled": are_tags_enabled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
