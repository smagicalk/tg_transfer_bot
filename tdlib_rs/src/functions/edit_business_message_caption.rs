use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits the caption of a message sent on behalf of a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection on behalf of which the message was sent
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `caption` - New message content caption; pass null to remove caption; 0-getOption("message_caption_length_max") characters
/// * `show_caption_above_media` - Pass true to show the caption above the media; otherwise, the caption will be shown below the media. May be true only for animation, photo, and video messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_message_caption(
    business_connection_id: String,
    chat_id: i64,
    message_id: i64,
    reply_markup: Option<crate::enums::ReplyMarkup>,
    caption: Option<crate::types::FormattedText>,
    show_caption_above_media: bool,
    client_id: i32,
) -> Result<crate::enums::BusinessMessage, crate::types::Error> {
    let request = json!({
    "@type": "editBusinessMessageCaption",
    "business_connection_id": business_connection_id,
    "chat_id": chat_id,
    "message_id": message_id,
    "reply_markup": reply_markup,
    "caption": caption,
    "show_caption_above_media": show_caption_above_media,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
