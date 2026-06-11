use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends a message on behalf of a business account; for bots only. Returns the message after it was sent
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request
/// * `chat_id` - Target chat
/// * `reply_to` - Information about the message to be replied; pass null if none
/// * `disable_notification` - Pass true to disable notification for the message
/// * `protect_content` - Pass true if the content of the message must be protected from forwarding and saving
/// * `effect_id` - Identifier of the effect to apply to the message
/// * `reply_markup` - Markup for replying to the message; pass null if none
/// * `input_message_content` - The content of the message to be sent
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_business_message(
    business_connection_id: String,
    chat_id: i64,
    reply_to: Option<crate::enums::InputMessageReplyTo>,
    disable_notification: bool,
    protect_content: bool,
    effect_id: i64,
    reply_markup: Option<crate::enums::ReplyMarkup>,
    input_message_content: crate::enums::InputMessageContent,
    client_id: i32,
) -> Result<crate::enums::BusinessMessage, crate::types::Error> {
    let request = json!({
    "@type": "sendBusinessMessage",
    "business_connection_id": business_connection_id,
    "chat_id": chat_id,
    "reply_to": reply_to,
    "disable_notification": disable_notification,
    "protect_content": protect_content,
    "effect_id": effect_id,
    "reply_markup": reply_markup,
    "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
