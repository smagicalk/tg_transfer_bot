#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Edits a subscription invite link for a channel chat. Requires can_invite_users right in the chat for own links and owner privileges for other links
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link to be edited
/// * `name` - Invite link name; 0-32 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_chat_subscription_invite_link(chat_id: i64, invite_link: String, name: String, client_id: i32) -> Result<crate::enums::ChatInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "editChatSubscriptionInviteLink",
        "chat_id": chat_id,
        "invite_link": invite_link,
        "name": name,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
