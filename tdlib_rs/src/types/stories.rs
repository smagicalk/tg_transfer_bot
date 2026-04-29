#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Stories {
    /// Approximate total number of stories found
    pub total_count: i32,
    /// The list of stories
    pub stories: Vec<crate::types::Story>,
    /// Identifiers of the pinned stories; returned only in getChatPostedToChatPageStories with from_story_id == 0
    pub pinned_story_ids: Vec<i32>,
}
