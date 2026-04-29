#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents link to a JPEG image
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultPhoto {
    /// Unique identifier of the query result
    pub id: String,
    /// Title of the result, if known
    pub title: String,
    /// A short description of the result, if known
    pub description: String,
    /// URL of the photo thumbnail, if it exists
    pub thumbnail_url: String,
    /// The URL of the JPEG photo (photo size must not exceed 5MB)
    pub photo_url: String,
    /// Width of the photo
    pub photo_width: i32,
    /// Height of the photo
    pub photo_height: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessagePhoto, inputMessageInvoice, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}
