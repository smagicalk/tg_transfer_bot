use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits the media content of a message with a text, an animation, an audio, a document, a photo or a video in an inline message sent via a bot; for bots only
/// # Arguments
/// * `inline_message_id` - Inline message identifier
/// * `reply_markup` - The new message reply markup; pass null if none; for bots only
/// * `input_message_content` - New content of the message. Must be one of the following types: inputMessageAnimation, inputMessageAudio, inputMessageDocument, inputMessagePhoto or inputMessageVideo
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_inline_message_media(
    inline_message_id: String,
    reply_markup: Option<crate::enums::ReplyMarkup>,
    input_message_content: crate::enums::InputMessageContent,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "editInlineMessageMedia",
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
