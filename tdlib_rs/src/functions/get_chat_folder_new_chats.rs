#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns new chats added to a shareable chat folder by its owner. The method must be called at most once in getOption("chat_folder_new_chats_update_period") for the given chat folder
/// # Arguments
/// * `chat_folder_id` - Chat folder identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_folder_new_chats(chat_folder_id: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getChatFolderNewChats",
        "chat_folder_id": chat_folder_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
