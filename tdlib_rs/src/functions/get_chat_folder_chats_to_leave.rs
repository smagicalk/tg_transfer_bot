use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns identifiers of pinned or always included chats from a chat folder, which are suggested to be left when the chat folder is deleted
/// # Arguments
/// * `chat_folder_id` - Chat folder identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_folder_chats_to_leave(
    chat_folder_id: i32,
    client_id: i32,
) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
    "@type": "getChatFolderChatsToLeave",
    "chat_folder_id": chat_folder_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
