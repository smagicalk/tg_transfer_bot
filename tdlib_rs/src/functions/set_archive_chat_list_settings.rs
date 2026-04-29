#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes settings for automatic moving of chats to and from the Archive chat lists
/// # Arguments
/// * `settings` - New settings
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_archive_chat_list_settings(settings: crate::types::ArchiveChatListSettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setArchiveChatListSettings",
        "settings": settings,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
