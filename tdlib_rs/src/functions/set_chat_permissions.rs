#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the chat members permissions. Supported only for basic groups and supergroups. Requires can_restrict_members administrator right
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `permissions` - New non-administrator members permissions in the chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_permissions(chat_id: i64, permissions: crate::types::ChatPermissions, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatPermissions",
        "chat_id": chat_id,
        "permissions": permissions,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
