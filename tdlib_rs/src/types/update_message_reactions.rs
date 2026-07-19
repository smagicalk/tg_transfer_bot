#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Reactions added to a message with anonymous reactions have changed; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageReactions {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// Point in time (Unix timestamp) when the reactions were changed
    pub date: i32,
    /// The list of reactions added to the message
    pub reactions: Vec<crate::types::MessageReaction>,
}
