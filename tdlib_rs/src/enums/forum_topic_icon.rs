#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ForumTopicIcon {
    /// Describes a forum topic icon
    #[serde(rename(serialize = "forumTopicIcon", deserialize = "forumTopicIcon"))]
    ForumTopicIcon(crate::types::ForumTopicIcon),
}
