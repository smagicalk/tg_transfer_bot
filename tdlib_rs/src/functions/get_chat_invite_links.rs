#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns invite links for a chat created by specified administrator. Requires administrator privileges and can_invite_users right in the chat to get own links and owner privileges to get other links
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `creator_user_id` - User identifier of a chat administrator. Must be an identifier of the current user for non-owner
/// * `is_revoked` - Pass true if revoked links needs to be returned instead of active or expired
/// * `offset_date` - Creation date of an invite link starting after which to return invite links; use 0 to get results from the beginning
/// * `offset_invite_link` - Invite link starting after which to return invite links; use empty string to get results from the beginning
/// * `limit` - The maximum number of invite links to return; up to 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_invite_links(chat_id: i64, creator_user_id: i64, is_revoked: bool, offset_date: i32, offset_invite_link: String, limit: i32, client_id: i32) -> Result<crate::enums::ChatInviteLinks, crate::types::Error> {
    let request = json!({
        "@type": "getChatInviteLinks",
        "chat_id": chat_id,
        "creator_user_id": creator_user_id,
        "is_revoked": is_revoked,
        "offset_date": offset_date,
        "offset_invite_link": offset_invite_link,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
