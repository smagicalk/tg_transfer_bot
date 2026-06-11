use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits the reply markup of a message sent on behalf of a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection on behalf of which the message was sent
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_message_reply_markup(
    business_connection_id: String,
    chat_id: i64,
    message_id: i64,
    reply_markup: Option<crate::enums::ReplyMarkup>,
    client_id: i32,
) -> Result<crate::enums::BusinessMessage, crate::types::Error> {
    let request = json!({
    "@type": "editBusinessMessageReplyMarkup",
    "business_connection_id": business_connection_id,
    "chat_id": chat_id,
    "message_id": message_id,
    "reply_markup": reply_markup,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
