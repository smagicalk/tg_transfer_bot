#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Creates a new invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `name` - Invite link name; 0-32 characters
/// * `expiration_date` - Point in time (Unix timestamp) when the link will expire; pass 0 if never
/// * `member_limit` - The maximum number of chat members that can join the chat via the link simultaneously; 0-99999; pass 0 if not limited
/// * `creates_join_request` - Pass true if users joining the chat via the link need to be approved by chat administrators. In this case, member_limit must be 0
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_chat_invite_link(chat_id: i64, name: String, expiration_date: i32, member_limit: i32, creates_join_request: bool, client_id: i32) -> Result<crate::enums::ChatInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "createChatInviteLink",
        "chat_id": chat_id,
        "name": name,
        "expiration_date": expiration_date,
        "member_limit": member_limit,
        "creates_join_request": creates_join_request,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
