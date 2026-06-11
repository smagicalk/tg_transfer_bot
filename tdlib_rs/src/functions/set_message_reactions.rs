use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets reactions on a message; for bots only
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `reaction_types` - Types of the reaction to set; pass an empty list to remove the reactions
/// * `is_big` - Pass true if the reactions are added with a big animation
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_message_reactions(
    chat_id: i64,
    message_id: i64,
    reaction_types: Vec<crate::enums::ReactionType>,
    is_big: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setMessageReactions",
    "chat_id": chat_id,
    "message_id": message_id,
    "reaction_types": reaction_types,
    "is_big": is_big,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
