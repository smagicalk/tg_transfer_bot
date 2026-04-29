#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The General forum topic was hidden or unhidden
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicToggleIsHidden {
    /// New information about the topic
    pub topic_info: crate::types::ForumTopicInfo,
}
