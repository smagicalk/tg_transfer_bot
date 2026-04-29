#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ForumTopics {
    /// Describes a list of forum topics
    #[serde(rename(serialize = "forumTopics", deserialize = "forumTopics"))]
    ForumTopics(crate::types::ForumTopics),
}
