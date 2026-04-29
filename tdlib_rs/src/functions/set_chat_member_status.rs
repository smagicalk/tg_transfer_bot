#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the status of a chat member; requires can_invite_users member right to add a chat member, can_promote_members administrator right to change administrator rights of the member,
/// and can_restrict_members administrator right to change restrictions of a user. This function is currently not suitable for transferring chat ownership; use transferChatOwnership instead.
/// Use addChatMember or banChatMember if some additional parameters needs to be passed
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `member_id` - Member identifier. Chats can be only banned and unbanned in supergroups and channels
/// * `status` - The new status of the member in the chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_member_status(chat_id: i64, member_id: crate::enums::MessageSender, status: crate::enums::ChatMemberStatus, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatMemberStatus",
        "chat_id": chat_id,
        "member_id": member_id,
        "status": status,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
