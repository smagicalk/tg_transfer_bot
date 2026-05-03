#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Number of chats in a story list has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateStoryListChatCount {
    /// The story list
    pub story_list: crate::enums::StoryList,
    /// Approximate total number of chats with active stories in the list
    pub chat_count: i32,
}
