#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of reactions added to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AddedReactions {
    /// The total number of found reactions
    pub total_count: i32,
    /// The list of added reactions
    pub reactions: Vec<crate::types::AddedReaction>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
