use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about a message with the callback button that originated a callback query; for bots only
/// # Arguments
/// * `chat_id` - Identifier of the chat the message belongs to
/// * `message_id` - Message identifier
/// * `callback_query_id` - Identifier of the callback query
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_callback_query_message(
    chat_id: i64,
    message_id: i64,
    callback_query_id: i64,
    client_id: i32,
) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
    "@type": "getCallbackQueryMessage",
    "chat_id": chat_id,
    "message_id": message_id,
    "callback_query_id": callback_query_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
