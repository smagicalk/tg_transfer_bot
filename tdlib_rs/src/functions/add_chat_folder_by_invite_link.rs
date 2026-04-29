#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds a chat folder by an invite link
/// # Arguments
/// * `invite_link` - Invite link for the chat folder
/// * `chat_ids` - Identifiers of the chats added to the chat folder. The chats are automatically joined if they aren't joined yet
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_chat_folder_by_invite_link(invite_link: String, chat_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addChatFolderByInviteLink",
        "invite_link": invite_link,
        "chat_ids": chat_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
