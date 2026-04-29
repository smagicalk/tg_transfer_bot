#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Only specific reactions are available in the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatAvailableReactionsSome {
    /// The list of reactions
    pub reactions: Vec<crate::enums::ReactionType>,
    /// The maximum allowed number of reactions per message; 1-11
    pub max_reaction_count: i32,
}
