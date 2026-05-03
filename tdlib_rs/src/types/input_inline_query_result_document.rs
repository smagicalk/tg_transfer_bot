#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a link to a file
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultDocument {
    /// Unique identifier of the query result
    pub id: String,
    /// Title of the resulting file
    pub title: String,
    /// Short description of the result, if known
    pub description: String,
    /// URL of the file
    pub document_url: String,
    /// MIME type of the file content; only "application/pdf" and "application/zip" are currently allowed
    pub mime_type: String,
    /// The URL of the file thumbnail, if it exists
    pub thumbnail_url: String,
    /// Width of the thumbnail
    pub thumbnail_width: i32,
    /// Height of the thumbnail
    pub thumbnail_height: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageDocument, inputMessageInvoice, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}
