#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A forum topic was deleted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicDeleted {
    /// Information about the topic
    pub topic_info: crate::types::ForumTopicInfo,
}
