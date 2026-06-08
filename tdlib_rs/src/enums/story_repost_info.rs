#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryRepostInfo {
    /// Contains information about original story that was reposted
    #[serde(rename(serialize = "storyRepostInfo", deserialize = "storyRepostInfo"))]
    StoryRepostInfo(crate::types::StoryRepostInfo),
}
