use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits the reply markup of an inline message sent via a bot; for bots only
/// # Arguments
/// * `inline_message_id` - Inline message identifier
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_inline_message_reply_markup(
    inline_message_id: String,
    reply_markup: Option<crate::enums::ReplyMarkup>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "editInlineMessageReplyMarkup",
    "inline_message_id": inline_message_id,
    "reply_markup": reply_markup,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
