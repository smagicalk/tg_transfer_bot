#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a link to a page containing an embedded video player or a video file
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultVideo {
    /// Unique identifier of the query result
    pub id: String,
    /// Title of the result
    pub title: String,
    /// A short description of the result, if known
    pub description: String,
    /// The URL of the video thumbnail (JPEG), if it exists
    pub thumbnail_url: String,
    /// URL of the embedded video player or video file
    pub video_url: String,
    /// MIME type of the content of the video URL, only "text/html" or "video/mp4" are currently supported
    pub mime_type: String,
    /// Width of the video
    pub video_width: i32,
    /// Height of the video
    pub video_height: i32,
    /// Video duration, in seconds
    pub video_duration: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageVideo, inputMessageInvoice, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}
