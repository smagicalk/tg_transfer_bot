use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Boosts a chat and returns the list of available chat boost slots for the current user after the boost
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `slot_ids` - Identifiers of boost slots of the current user from which to apply boosts to the chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn boost_chat(
    chat_id: i64,
    slot_ids: Vec<i32>,
    client_id: i32,
) -> Result<crate::enums::ChatBoostSlots, crate::types::Error> {
    let request = json!({
    "@type": "boostChat",
    "chat_id": chat_id,
    "slot_ids": slot_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
