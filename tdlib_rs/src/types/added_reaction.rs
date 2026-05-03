#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a reaction applied to a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddedReaction {
    /// Type of the reaction
    pub r#type: crate::enums::ReactionType,
    /// Identifier of the chat member, applied the reaction
    pub sender_id: crate::enums::MessageSender,
    /// True, if the reaction was added by the current user
    pub is_outgoing: bool,
    /// Point in time (Unix timestamp) when the reaction was added
    pub date: i32,
}
