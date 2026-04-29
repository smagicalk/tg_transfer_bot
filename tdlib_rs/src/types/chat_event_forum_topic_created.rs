#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A new forum topic was created
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicCreated {
    /// Information about the topic
    pub topic_info: crate::types::ForumTopicInfo,
}
