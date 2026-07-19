#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// User changed its reactions on a message with public reactions; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageReaction {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// Identifier of the user or chat that changed reactions
    pub actor_id: crate::enums::MessageSender,
    /// Point in time (Unix timestamp) when the reactions were changed
    pub date: i32,
    /// Old list of chosen reactions
    pub old_reaction_types: Vec<crate::enums::ReactionType>,
    /// New list of chosen reactions
    pub new_reaction_types: Vec<crate::enums::ReactionType>,
}
