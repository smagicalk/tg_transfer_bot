use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits the text of an inline text or game message sent via a bot; for bots only
/// # Arguments
/// * `inline_message_id` - Inline message identifier
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `input_message_content` - New text content of the message. Must be of type inputMessageText
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_inline_message_text(
    inline_message_id: String,
    reply_markup: Option<crate::enums::ReplyMarkup>,
    input_message_content: crate::enums::InputMessageContent,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "editInlineMessageText",
    "inline_message_id": inline_message_id,
    "reply_markup": reply_markup,
    "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
