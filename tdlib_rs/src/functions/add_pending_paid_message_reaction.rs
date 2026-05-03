use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds the paid message reaction to a message. Use getMessageAvailableReactions to check whether the reaction is available for the message
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `star_count` - Number of Telegram Stars to be used for the reaction. The total number of pending paid reactions must not exceed getOption("paid_reaction_star_count_max")
/// * `r#type` - Type of the paid reaction; pass null if the user didn't choose reaction type explicitly, for example, the reaction is set from the message bubble
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_pending_paid_message_reaction(
    chat_id: i64,
    message_id: i64,
    star_count: i64,
    r#type: Option<crate::enums::PaidReactionType>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "addPendingPaidMessageReaction",
    "chat_id": chat_id,
    "message_id": message_id,
    "star_count": star_count,
    "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
