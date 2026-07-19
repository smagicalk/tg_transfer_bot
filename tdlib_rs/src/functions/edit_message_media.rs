use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits the media content of a message, including message caption. If only the caption needs to be edited, use editMessageCaption instead.
/// The type of message content in an album can't be changed with exception of replacing a photo with a video or vice versa. Returns the edited message after the edit is completed on the server side
/// # Arguments
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_edit_media to check whether the message can be edited
/// * `reply_markup` - The new message reply markup; pass null if none; for bots only
/// * `input_message_content` - New content of the message. Must be one of the following types: inputMessageAnimation, inputMessageAudio, inputMessageDocument, inputMessagePhoto or inputMessageVideo
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_media(
    chat_id: i64,
    message_id: i64,
    reply_markup: Option<crate::enums::ReplyMarkup>,
    input_message_content: crate::enums::InputMessageContent,
    client_id: i32,
) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
    "@type": "editMessageMedia",
    "chat_id": chat_id,
    "message_id": message_id,
    "reply_markup": reply_markup,
    "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
