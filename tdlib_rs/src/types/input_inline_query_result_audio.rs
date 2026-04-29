#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a link to an MP3 audio file
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultAudio {
    /// Unique identifier of the query result
    pub id: String,
    /// Title of the audio file
    pub title: String,
    /// Performer of the audio file
    pub performer: String,
    /// The URL of the audio file
    pub audio_url: String,
    /// Audio file duration, in seconds
    pub audio_duration: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageAudio, inputMessageInvoice, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}
