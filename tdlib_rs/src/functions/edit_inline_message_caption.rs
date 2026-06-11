use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits the caption of an inline message sent via a bot; for bots only
/// # Arguments
/// * `inline_message_id` - Inline message identifier
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `caption` - New message content caption; pass null to remove caption; 0-getOption("message_caption_length_max") characters
/// * `show_caption_above_media` - Pass true to show the caption above the media; otherwise, the caption will be shown below the media. May be true only for animation, photo, and video messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_inline_message_caption(
    inline_message_id: String,
    reply_markup: Option<crate::enums::ReplyMarkup>,
    caption: Option<crate::types::FormattedText>,
    show_caption_above_media: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "editInlineMessageCaption",
    "inline_message_id": inline_message_id,
    "reply_markup": reply_markup,
    "caption": caption,
    "show_caption_above_media": show_caption_above_media,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
