#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Creates new chat folder. Returns information about the created chat folder. There can be up to getOption("chat_folder_count_max") chat folders, but the limit can be increased with Telegram Premium
/// # Arguments
/// * `folder` - The new chat folder
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_chat_folder(folder: crate::types::ChatFolder, client_id: i32) -> Result<crate::enums::ChatFolderInfo, crate::types::Error> {
    let request = json!({
        "@type": "createChatFolder",
        "folder": folder,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
