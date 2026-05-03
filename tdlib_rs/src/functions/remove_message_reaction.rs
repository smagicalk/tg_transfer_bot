use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes a reaction from a message. A chosen reaction can always be removed
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `reaction_type` - Type of the reaction to remove. The paid reaction can't be removed
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_message_reaction(
    chat_id: i64,
    message_id: i64,
    reaction_type: crate::enums::ReactionType,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "removeMessageReaction",
    "chat_id": chat_id,
    "message_id": message_id,
    "reaction_type": reaction_type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
