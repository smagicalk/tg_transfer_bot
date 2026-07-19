use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side
/// # Arguments
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_be_edited to check whether the message can be edited
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_reply_markup(
    chat_id: i64,
    message_id: i64,
    reply_markup: Option<crate::enums::ReplyMarkup>,
    client_id: i32,
) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
    "@type": "editMessageReplyMarkup",
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
