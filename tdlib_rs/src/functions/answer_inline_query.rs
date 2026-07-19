use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets the result of an inline query; for bots only
/// # Arguments
/// * `inline_query_id` - Identifier of the inline query
/// * `is_personal` - Pass true if results may be cached and returned only for the user who sent the query. By default, results may be returned to any user who sends the same query
/// * `button` - Button to be shown above inline query results; pass null if none
/// * `results` - The results of the query
/// * `cache_time` - Allowed time to cache the results of the query, in seconds
/// * `next_offset` - Offset for the next inline query; pass an empty string if there are no more results
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn answer_inline_query(
    inline_query_id: i64,
    is_personal: bool,
    button: Option<crate::types::InlineQueryResultsButton>,
    results: Vec<crate::enums::InputInlineQueryResult>,
    cache_time: i32,
    next_offset: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "answerInlineQuery",
    "inline_query_id": inline_query_id,
    "is_personal": is_personal,
    "button": button,
    "results": results,
    "cache_time": cache_time,
    "next_offset": next_offset,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
