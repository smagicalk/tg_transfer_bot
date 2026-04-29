#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns approximate number of chats in a being created chat folder. Main and archive chat lists must be fully preloaded for this function to work correctly
/// # Arguments
/// * `folder` - The new chat folder
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_folder_chat_count(folder: crate::types::ChatFolder, client_id: i32) -> Result<crate::enums::Count, crate::types::Error> {
    let request = json!({
        "@type": "getChatFolderChatCount",
        "folder": folder,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
