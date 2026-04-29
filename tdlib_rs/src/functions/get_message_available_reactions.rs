#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns reactions, which can be added to a message. The list can change after updateActiveEmojiReactions, updateChatAvailableReactions for the chat, or updateMessageInteractionInfo for the message
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `row_size` - Number of reaction per row, 5-25
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_available_reactions(chat_id: i64, message_id: i64, row_size: i32, client_id: i32) -> Result<crate::enums::AvailableReactions, crate::types::Error> {
    let request = json!({
        "@type": "getMessageAvailableReactions",
        "chat_id": chat_id,
        "message_id": message_id,
        "row_size": row_size,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
