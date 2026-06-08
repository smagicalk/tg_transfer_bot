#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An automatically scheduled message with video has been successfully sent after conversion
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateVideoPublished {
    /// Identifier of the chat with the message
    pub chat_id: i64,
    /// Identifier of the sent message
    pub message_id: i64,
}
