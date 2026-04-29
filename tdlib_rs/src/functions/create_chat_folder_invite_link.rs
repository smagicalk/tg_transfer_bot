#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Creates a new invite link for a chat folder. A link can be created for a chat folder if it has only pinned and included chats
/// # Arguments
/// * `chat_folder_id` - Chat folder identifier
/// * `name` - Name of the link; 0-32 characters
/// * `chat_ids` - Identifiers of chats to be accessible by the invite link. Use getChatsForChatFolderInviteLink to get suitable chats. Basic groups will be automatically converted to supergroups before link creation
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_chat_folder_invite_link(chat_folder_id: i32, name: String, chat_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::ChatFolderInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "createChatFolderInviteLink",
        "chat_folder_id": chat_folder_id,
        "name": name,
        "chat_ids": chat_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
