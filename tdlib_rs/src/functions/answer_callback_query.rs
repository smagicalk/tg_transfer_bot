use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets the result of a callback query; for bots only
/// # Arguments
/// * `callback_query_id` - Identifier of the callback query
/// * `text` - Text of the answer
/// * `show_alert` - Pass true to show an alert to the user instead of a toast notification
/// * `url` - URL to be opened
/// * `cache_time` - Time during which the result of the query can be cached, in seconds
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn answer_callback_query(
    callback_query_id: i64,
    text: String,
    show_alert: bool,
    url: String,
    cache_time: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "answerCallbackQuery",
    "callback_query_id": callback_query_id,
    "text": text,
    "show_alert": show_alert,
    "url": url,
    "cache_time": cache_time,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
