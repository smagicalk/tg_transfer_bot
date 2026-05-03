use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Saves an inline message to be sent by the given user
/// # Arguments
/// * `bot_user_id` - Identifier of the bot that created the message
/// * `prepared_message_id` - Identifier of the prepared message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_prepared_inline_message(
    bot_user_id: i64,
    prepared_message_id: String,
    client_id: i32,
) -> Result<crate::enums::PreparedInlineMessage, crate::types::Error> {
    let request = json!({
    "@type": "getPreparedInlineMessage",
    "bot_user_id": bot_user_id,
    "prepared_message_id": prepared_message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
