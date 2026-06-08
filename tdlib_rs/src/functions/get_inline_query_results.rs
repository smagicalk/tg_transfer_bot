use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot
/// * `chat_id` - Identifier of the chat where the query was sent
/// * `user_location` - Location of the user; pass null if unknown or the bot doesn't need user's location
/// * `query` - Text of the query
/// * `offset` - Offset of the first entry to return; use empty string to get the first chunk of results
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_inline_query_results(
    bot_user_id: i64,
    chat_id: i64,
    user_location: Option<crate::types::Location>,
    query: String,
    offset: String,
    client_id: i32,
) -> Result<crate::enums::InlineQueryResults, crate::types::Error> {
    let request = json!({
    "@type": "getInlineQueryResults",
    "bot_user_id": bot_user_id,
    "chat_id": chat_id,
    "user_location": user_location,
    "query": query,
    "offset": offset,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
