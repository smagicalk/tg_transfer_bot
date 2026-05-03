#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryInfo {
    /// Contains basic information about a story
    #[serde(rename(serialize = "storyInfo", deserialize = "storyInfo"))]
    StoryInfo(crate::types::StoryInfo),
}
