#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A story became inaccessible
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateStoryDeleted {
    /// Identifier of the chat that posted the story
    pub story_poster_chat_id: i64,
    /// Story identifier
    pub story_id: i32,
}
