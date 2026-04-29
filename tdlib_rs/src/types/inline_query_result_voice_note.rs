#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a voice note
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultVoiceNote {
    /// Unique identifier of the query result
    pub id: String,
    /// Voice note
    pub voice_note: crate::types::VoiceNote,
    /// Title of the voice note
    pub title: String,
}
