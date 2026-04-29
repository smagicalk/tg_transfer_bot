#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of interactions with a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryInteractions {
    /// Approximate total number of interactions found
    pub total_count: i32,
    /// Approximate total number of found forwards and reposts; always 0 for chat stories
    pub total_forward_count: i32,
    /// Approximate total number of found reactions; always 0 for chat stories
    pub total_reaction_count: i32,
    /// List of story interactions
    pub interactions: Vec<crate::types::StoryInteraction>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
