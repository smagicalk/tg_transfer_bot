#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes active stories posted by a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActiveStories {
    /// Identifier of the chat that posted the stories
    pub chat_id: i64,
    /// Identifier of the story list in which the stories are shown; may be null if the stories aren't shown in a story list
    pub list: Option<crate::enums::StoryList>,
    /// A parameter used to determine order of the stories in the story list; 0 if the stories doesn't need to be shown in the story list. Stories must be sorted by the pair (order, story_poster_chat_id) in descending order
    pub order: i64,
    /// True, if the stories are shown in the main story list and can be archived; otherwise, the stories can be hidden from the main story list
    /// only by calling removeTopChat with topChatCategoryUsers and the chat_id. Stories of the current user can't be archived nor hidden using removeTopChat
    pub can_be_archived: bool,
    /// Identifier of the last read active story
    pub max_read_story_id: i32,
    /// Basic information about the stories; use getStory to get full information about the stories. The stories are in chronological order (i.e., in order of increasing story identifiers)
    pub stories: Vec<crate::types::StoryInfo>,
}
