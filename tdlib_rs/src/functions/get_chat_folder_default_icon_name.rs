use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns default icon name for a folder. Can be called synchronously
/// # Arguments
/// * `folder` - Chat folder
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_folder_default_icon_name(
    folder: crate::types::ChatFolder,
    client_id: i32,
) -> Result<crate::enums::ChatFolderIcon, crate::types::Error> {
    let request = json!({
    "@type": "getChatFolderDefaultIconName",
    "folder": folder,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
