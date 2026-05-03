#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a link to an opus-encoded audio file within an OGG container, single channel audio
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultVoiceNote {
    /// Unique identifier of the query result
    pub id: String,
    /// Title of the voice note
    pub title: String,
    /// The URL of the voice note file
    pub voice_note_url: String,
    /// Duration of the voice note, in seconds
    pub voice_note_duration: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageVoiceNote, inputMessageInvoice, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}
