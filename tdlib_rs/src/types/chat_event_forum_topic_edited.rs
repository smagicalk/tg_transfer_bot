#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A forum topic was edited
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicEdited {
    /// Old information about the topic
    pub old_topic_info: crate::types::ForumTopicInfo,
    /// New information about the topic
    pub new_topic_info: crate::types::ForumTopicInfo,
}
