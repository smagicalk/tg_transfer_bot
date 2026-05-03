use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Bans a member in a chat; requires can_restrict_members administrator right. Members can't be banned in private or secret chats. In supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless unbanned first
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `member_id` - Member identifier
/// * `banned_until_date` - Point in time (Unix timestamp) when the user will be unbanned; 0 if never. If the user is banned for more than 366 days or for less than 30 seconds from the current time, the user is considered to be banned forever. Ignored in basic groups and if a chat is banned
/// * `revoke_messages` - Pass true to delete all messages in the chat for the user who is being removed. Always true for supergroups and channels
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn ban_chat_member(
    chat_id: i64,
    member_id: crate::enums::MessageSender,
    banned_until_date: i32,
    revoke_messages: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "banChatMember",
    "chat_id": chat_id,
    "member_id": member_id,
    "banned_until_date": banned_until_date,
    "revoke_messages": revoke_messages,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
