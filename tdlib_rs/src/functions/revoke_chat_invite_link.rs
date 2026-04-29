#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Revokes invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links.
/// If a primary link is revoked, then additionally to the revoked link returns new primary link
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link to be revoked
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn revoke_chat_invite_link(chat_id: i64, invite_link: String, client_id: i32) -> Result<crate::enums::ChatInviteLinks, crate::types::Error> {
    let request = json!({
        "@type": "revokeChatInviteLink",
        "chat_id": chat_id,
        "invite_link": invite_link,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
