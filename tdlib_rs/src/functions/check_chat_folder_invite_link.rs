use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks the validity of an invite link for a chat folder and returns information about the corresponding chat folder
/// # Arguments
/// * `invite_link` - Invite link to be checked
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_chat_folder_invite_link(
    invite_link: String,
    client_id: i32,
) -> Result<crate::enums::ChatFolderInviteLinkInfo, crate::types::Error> {
    let request = json!({
    "@type": "checkChatFolderInviteLink",
    "invite_link": invite_link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
