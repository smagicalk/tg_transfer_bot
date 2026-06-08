use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds a reaction or a tag to a message. Use getMessageAvailableReactions to receive the list of available reactions for the message
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `reaction_type` - Type of the reaction to add. Use addPendingPaidMessageReaction instead to add the paid reaction
/// * `is_big` - Pass true if the reaction is added with a big animation
/// * `update_recent_reactions` - Pass true if the reaction needs to be added to recent reactions; tags are never added to the list of recent reactions
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_message_reaction(
    chat_id: i64,
    message_id: i64,
    reaction_type: crate::enums::ReactionType,
    is_big: bool,
    update_recent_reactions: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "addMessageReaction",
    "chat_id": chat_id,
    "message_id": message_id,
    "reaction_type": reaction_type,
    "is_big": is_big,
    "update_recent_reactions": update_recent_reactions,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
