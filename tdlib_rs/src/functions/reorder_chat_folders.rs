#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the order of chat folders
/// # Arguments
/// * `chat_folder_ids` - Identifiers of chat folders in the new correct order
/// * `main_chat_list_position` - Position of the main chat list among chat folders, 0-based. Can be non-zero only for Premium users
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reorder_chat_folders(chat_folder_ids: Vec<i32>, main_chat_list_position: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderChatFolders",
        "chat_folder_ids": chat_folder_ids,
        "main_chat_list_position": main_chat_list_position,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
