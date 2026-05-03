use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about a giveaway
/// # Arguments
/// * `chat_id` - Identifier of the channel chat which started the giveaway
/// * `message_id` - Identifier of the giveaway or a giveaway winners message in the chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_giveaway_info(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> Result<crate::enums::GiveawayInfo, crate::types::Error> {
    let request = json!({
    "@type": "getGiveawayInfo",
    "chat_id": chat_id,
    "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
