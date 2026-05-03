use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds a new member to a chat; requires can_invite_users member right. Members can't be added to private or secret chats. Returns information about members that weren't added
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `user_id` - Identifier of the user
/// * `forward_limit` - The number of earlier messages from the chat to be forwarded to the new member; up to 100. Ignored for supergroups and channels, or if the added user is a bot
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_chat_member(
    chat_id: i64,
    user_id: i64,
    forward_limit: i32,
    client_id: i32,
) -> Result<crate::enums::FailedToAddMembers, crate::types::Error> {
    let request = json!({
    "@type": "addChatMember",
    "chat_id": chat_id,
    "user_id": user_id,
    "forward_limit": forward_limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
