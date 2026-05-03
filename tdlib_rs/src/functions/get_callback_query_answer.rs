use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
/// # Arguments
/// * `chat_id` - Identifier of the chat with the message
/// * `message_id` - Identifier of the message from which the query originated. The message must not be scheduled
/// * `payload` - Query payload
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_callback_query_answer(
    chat_id: i64,
    message_id: i64,
    payload: crate::enums::CallbackQueryPayload,
    client_id: i32,
) -> Result<crate::enums::CallbackQueryAnswer, crate::types::Error> {
    let request = json!({
    "@type": "getCallbackQueryAnswer",
    "chat_id": chat_id,
    "message_id": message_id,
    "payload": payload,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
