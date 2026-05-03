use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns settings for automatic moving of chats to and from the Archive chat lists
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_archive_chat_list_settings(
    client_id: i32,
) -> Result<crate::enums::ArchiveChatListSettings, crate::types::Error> {
    let request = json!({
    "@type": "getArchiveChatListSettings",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
