use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns reactions added for a message, along with their sender
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message. Use message.interaction_info.reactions.can_get_added_reactions to check whether added reactions can be received for the message
/// * `reaction_type` - Type of the reactions to return; pass null to return all added reactions; reactionTypePaid isn't supported
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of reactions to be returned; must be positive and can't be greater than 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_added_reactions(
    chat_id: i64,
    message_id: i64,
    reaction_type: Option<crate::enums::ReactionType>,
    offset: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::AddedReactions, crate::types::Error> {
    let request = json!({
    "@type": "getMessageAddedReactions",
    "chat_id": chat_id,
    "message_id": message_id,
    "reaction_type": reaction_type,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
