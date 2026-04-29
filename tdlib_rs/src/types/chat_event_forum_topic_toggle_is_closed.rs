#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A forum topic was closed or reopened
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicToggleIsClosed {
    /// New information about the topic
    pub topic_info: crate::types::ForumTopicInfo,
}
