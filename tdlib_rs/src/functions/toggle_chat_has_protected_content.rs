#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the ability of users to save, forward, or copy chat content. Requires owner privileges in basic groups, supergroups and channels.
/// Requires Telegram Premium to enable protected content in private chats. Not available in Saved Messages and private chats with bots or support accounts
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `has_protected_content` - New value of has_protected_content
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_has_protected_content(chat_id: i64, has_protected_content: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatHasProtectedContent",
        "chat_id": chat_id,
        "has_protected_content": has_protected_content,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
