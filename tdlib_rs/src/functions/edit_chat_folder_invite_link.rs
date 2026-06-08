use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits an invite link for a chat folder
/// # Arguments
/// * `chat_folder_id` - Chat folder identifier
/// * `invite_link` - Invite link to be edited
/// * `name` - New name of the link; 0-32 characters
/// * `chat_ids` - New identifiers of chats to be accessible by the invite link. Use getChatsForChatFolderInviteLink to get suitable chats. Basic groups will be automatically converted to supergroups before link editing
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_chat_folder_invite_link(
    chat_folder_id: i32,
    invite_link: String,
    name: String,
    chat_ids: Vec<i64>,
    client_id: i32,
) -> Result<crate::enums::ChatFolderInviteLink, crate::types::Error> {
    let request = json!({
    "@type": "editChatFolderInviteLink",
    "chat_folder_id": chat_folder_id,
    "invite_link": invite_link,
    "name": name,
    "chat_ids": chat_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
