#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a point on the map
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultLocation {
    /// Unique identifier of the query result
    pub id: String,
    /// Location result
    pub location: crate::types::Location,
    /// Amount of time relative to the message sent time until the location can be updated, in seconds
    pub live_period: i32,
    /// Title of the result
    pub title: String,
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
