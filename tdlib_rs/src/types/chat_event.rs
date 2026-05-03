#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a chat event
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEvent {
    /// Chat event identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Point in time (Unix timestamp) when the event happened
    pub date: i32,
    /// Identifier of the user or chat who performed the action
    pub member_id: crate::enums::MessageSender,
    /// The action
    pub action: crate::enums::ChatEventAction,
}
