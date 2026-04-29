#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes revoked chat invite links. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link to revoke
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_revoked_chat_invite_link(chat_id: i64, invite_link: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteRevokedChatInviteLink",
        "chat_id": chat_id,
        "invite_link": invite_link,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
