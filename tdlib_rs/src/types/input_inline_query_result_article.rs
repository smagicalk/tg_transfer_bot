#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a link to an article or web page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultArticle {
    /// Unique identifier of the query result
    pub id: String,
    /// URL of the result, if it exists
    pub url: String,
    /// Title of the result
    pub title: String,
    /// A short description of the result
    pub description: String,
    /// URL of the result thumbnail, if it exists
    pub thumbnail_url: String,
    /// Thumbnail width, if known
    pub thumbnail_width: i32,
    /// Thumbnail height, if known
    pub thumbnail_height: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageInvoice, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}
