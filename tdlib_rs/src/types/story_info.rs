#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains basic information about a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryInfo {
    /// Unique story identifier among stories of the chat
    pub story_id: i32,
    /// Point in time (Unix timestamp) when the story was published
    pub date: i32,
    /// True, if the story is available only to close friends
    pub is_for_close_friends: bool,
    /// True, if the story is a live story
    pub is_live: bool,
}
