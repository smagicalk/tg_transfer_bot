#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ForumTopicInfo {
    /// Contains basic information about a forum topic
    #[serde(rename(serialize = "forumTopicInfo", deserialize = "forumTopicInfo"))]
    ForumTopicInfo(crate::types::ForumTopicInfo),
}
