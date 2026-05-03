use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Process new chats added to a shareable chat folder by its owner
/// # Arguments
/// * `chat_folder_id` - Chat folder identifier
/// * `added_chat_ids` - Identifiers of the new chats, which are added to the chat folder. The chats are automatically joined if they aren't joined yet
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn process_chat_folder_new_chats(
    chat_folder_id: i32,
    added_chat_ids: Vec<i64>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "processChatFolderNewChats",
    "chat_folder_id": chat_folder_id,
    "added_chat_ids": added_chat_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
