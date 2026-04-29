#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents an audio file
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultAudio {
    /// Unique identifier of the query result
    pub id: String,
    /// Audio file
    pub audio: crate::types::Audio,
}
