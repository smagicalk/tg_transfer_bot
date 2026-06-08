use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns approximate number of bots similar to the given bot
/// # Arguments
/// * `bot_user_id` - User identifier of the target bot
/// * `return_local` - Pass true to get the number of bots without sending network requests, or -1 if the number of bots is unknown locally
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_bot_similar_bot_count(
    bot_user_id: i64,
    return_local: bool,
    client_id: i32,
) -> Result<crate::enums::Count, crate::types::Error> {
    let request = json!({
    "@type": "getBotSimilarBotCount",
    "bot_user_id": bot_user_id,
    "return_local": return_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
