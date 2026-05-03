use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits existing chat folder. Returns information about the edited chat folder
/// # Arguments
/// * `chat_folder_id` - Chat folder identifier
/// * `folder` - The edited chat folder
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_chat_folder(
    chat_folder_id: i32,
    folder: crate::types::ChatFolder,
    client_id: i32,
) -> Result<crate::enums::ChatFolderInfo, crate::types::Error> {
    let request = json!({
    "@type": "editChatFolder",
    "chat_folder_id": chat_folder_id,
    "folder": folder,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
