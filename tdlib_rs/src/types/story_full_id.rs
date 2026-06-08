#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains identifier of a story along with identifier of the chat that posted it
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryFullId {
    /// Identifier of the chat that posted the story
    pub poster_chat_id: i64,
    /// Unique story identifier among stories of the chat
    pub story_id: i32,
}
