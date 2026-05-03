use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes all revoked chat invite links created by a given chat administrator. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `creator_user_id` - User identifier of a chat administrator, which links will be deleted. Must be an identifier of the current user for non-owner
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_all_revoked_chat_invite_links(
    chat_id: i64,
    creator_user_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteAllRevokedChatInviteLinks",
    "chat_id": chat_id,
    "creator_user_id": creator_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
