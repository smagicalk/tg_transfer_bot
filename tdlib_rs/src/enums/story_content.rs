#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryContent {
    /// A photo story
    #[serde(rename(serialize = "storyContentPhoto", deserialize = "storyContentPhoto"))]
    Photo(crate::types::StoryContentPhoto),
    /// A video story
    #[serde(rename(serialize = "storyContentVideo", deserialize = "storyContentVideo"))]
    Video(crate::types::StoryContentVideo),
    /// A live story
    #[serde(rename(serialize = "storyContentLive", deserialize = "storyContentLive"))]
    Live(crate::types::StoryContentLive),
    /// A story content that is not supported in the current TDLib version
    #[serde(rename(
        serialize = "storyContentUnsupported",
        deserialize = "storyContentUnsupported"
    ))]
    Unsupported,
}
