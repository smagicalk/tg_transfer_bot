use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Uses an invite link to add the current user to the chat if possible. May return an error with a message "INVITE_REQUEST_SENT" if only a join request was created
/// # Arguments
/// * `invite_link` - Invite link to use
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn join_chat_by_invite_link(
    invite_link: String,
    client_id: i32,
) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
    "@type": "joinChatByInviteLink",
    "invite_link": invite_link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
