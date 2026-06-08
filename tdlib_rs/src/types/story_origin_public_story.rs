#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The original story was a public story that was posted by a known chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryOriginPublicStory {
    /// Identifier of the chat that posted original story
    pub chat_id: i64,
    /// Story identifier of the original story
    pub story_id: i32,
}
