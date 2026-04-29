#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of boosts applied to a chat; requires administrator rights in the chat
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `only_gift_codes` - Pass true to receive only boosts received from gift codes and giveaways created by the chat
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of boosts to be returned; up to 100. For optimal performance, the number of returned boosts can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boosts(chat_id: i64, only_gift_codes: bool, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundChatBoosts, crate::types::Error> {
    let request = json!({
        "@type": "getChatBoosts",
        "chat_id": chat_id,
        "only_gift_codes": only_gift_codes,
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
