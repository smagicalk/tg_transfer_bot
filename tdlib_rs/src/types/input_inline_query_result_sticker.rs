#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a link to a WEBP, TGS, or WEBM sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultSticker {
    /// Unique identifier of the query result
    pub id: String,
    /// URL of the sticker thumbnail, if it exists
    pub thumbnail_url: String,
    /// The URL of the WEBP, TGS, or WEBM sticker (sticker file size must not exceed 5MB)
    pub sticker_url: String,
    /// Width of the sticker
    pub sticker_width: i32,
    /// Height of the sticker
    pub sticker_height: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageSticker, inputMessageInvoice, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}
