#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a list of forum topics
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ForumTopics {
    /// Approximate total number of forum topics found
    pub total_count: i32,
    /// List of forum topics
    pub topics: Vec<crate::types::ForumTopic>,
    /// Offset date for the next getForumTopics request
    pub next_offset_date: i32,
    /// Offset message identifier for the next getForumTopics request
    pub next_offset_message_id: i64,
    /// Offset forum topic identifier for the next getForumTopics request
    pub next_offset_forum_topic_id: i32,
}
