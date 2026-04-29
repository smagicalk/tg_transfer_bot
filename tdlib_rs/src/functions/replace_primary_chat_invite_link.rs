#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Replaces current primary invite link for a chat with a new primary invite link. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn replace_primary_chat_invite_link(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "replacePrimaryChatInviteLink",
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
