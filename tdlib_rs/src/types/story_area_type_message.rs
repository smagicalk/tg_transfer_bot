#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An area pointing to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeMessage {
    /// Identifier of the chat with the message
    pub chat_id: i64,
    /// Identifier of the message
    pub message_id: i64,
}
