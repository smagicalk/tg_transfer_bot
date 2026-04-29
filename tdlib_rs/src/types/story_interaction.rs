#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents interaction with a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryInteraction {
    /// Identifier of the user or chat that made the interaction
    pub actor_id: crate::enums::MessageSender,
    /// Approximate point in time (Unix timestamp) when the interaction happened
    pub interaction_date: i32,
    /// Block list to which the actor is added; may be null if none or for chat stories
    pub block_list: Option<crate::enums::BlockList>,
    /// Type of the interaction
    pub r#type: crate::enums::StoryInteractionType,
}
