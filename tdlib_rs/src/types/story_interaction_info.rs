#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about interactions with a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryInteractionInfo {
    /// Number of times the story was viewed
    pub view_count: i32,
    /// Number of times the story was forwarded; 0 if none or unknown
    pub forward_count: i32,
    /// Number of reactions added to the story; 0 if none or unknown
    pub reaction_count: i32,
    /// Identifiers of at most 3 recent viewers of the story
    pub recent_viewer_user_ids: Vec<i64>,
}
