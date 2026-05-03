#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Basic information about a topic in a forum chat was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateForumTopicInfo {
    /// New information about the topic
    pub info: crate::types::ForumTopicInfo,
}
