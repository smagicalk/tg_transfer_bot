#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ForumTopic {
    /// Describes a forum topic
    #[serde(rename(serialize = "forumTopic", deserialize = "forumTopic"))]
    ForumTopic(crate::types::ForumTopic),
}
