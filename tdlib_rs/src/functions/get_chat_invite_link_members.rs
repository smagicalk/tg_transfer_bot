use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns chat members joined a chat via an invite link. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link for which to return chat members
/// * `only_with_expired_subscription` - Pass true if the link is a subscription link and only members with expired subscription must be returned
/// * `offset_member` - A chat member from which to return next chat members; pass null to get results from the beginning
/// * `limit` - The maximum number of chat members to return; up to 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_invite_link_members(
    chat_id: i64,
    invite_link: String,
    only_with_expired_subscription: bool,
    offset_member: Option<crate::types::ChatInviteLinkMember>,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::ChatInviteLinkMembers, crate::types::Error> {
    let request = json!({
    "@type": "getChatInviteLinkMembers",
    "chat_id": chat_id,
    "invite_link": invite_link,
    "only_with_expired_subscription": only_with_expired_subscription,
    "offset_member": offset_member,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
