#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a link to an animated GIF or an animated (i.e., without sound) H.264/MPEG-4 AVC video
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultAnimation {
    /// Unique identifier of the query result
    pub id: String,
    /// Title of the query result
    pub title: String,
    /// URL of the result thumbnail (JPEG, GIF, or MPEG4), if it exists
    pub thumbnail_url: String,
    /// MIME type of the video thumbnail. If non-empty, must be one of "image/jpeg", "image/gif" and "video/mp4"
    pub thumbnail_mime_type: String,
    /// The URL of the video file (file size must not exceed 1MB)
    pub video_url: String,
    /// MIME type of the video file. Must be one of "image/gif" and "video/mp4"
    pub video_mime_type: String,
    /// Duration of the video, in seconds
    pub video_duration: i32,
    /// Width of the video
    pub video_width: i32,
    /// Height of the video
    pub video_height: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageAnimation, inputMessageInvoice, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}
