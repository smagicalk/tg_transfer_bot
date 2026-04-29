#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes an invite link for a chat folder
/// # Arguments
/// * `chat_folder_id` - Chat folder identifier
/// * `invite_link` - Invite link to be deleted
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_folder_invite_link(chat_folder_id: i32, invite_link: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatFolderInviteLink",
        "chat_folder_id": chat_folder_id,
        "invite_link": invite_link,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
