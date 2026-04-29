#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A pinned forum topic was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicPinned {
    /// Information about the old pinned topic; may be null
    pub old_topic_info: Option<crate::types::ForumTopicInfo>,
    /// Information about the new pinned topic; may be null
    pub new_topic_info: Option<crate::types::ForumTopicInfo>,
}
