#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes reactions, available in a chat. Available for basic groups, supergroups, and channels. Requires can_change_info member right
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `available_reactions` - Reactions available in the chat. All explicitly specified emoji reactions must be active. In channel chats up to the chat's boost level custom emoji reactions can be explicitly specified
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_available_reactions(chat_id: i64, available_reactions: crate::enums::ChatAvailableReactions, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatAvailableReactions",
        "chat_id": chat_id,
        "available_reactions": available_reactions,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
