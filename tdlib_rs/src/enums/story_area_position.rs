#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryAreaPosition {
    /// Describes position of a clickable rectangle area on a story media
    #[serde(rename(serialize = "storyAreaPosition", deserialize = "storyAreaPosition"))]
    StoryAreaPosition(crate::types::StoryAreaPosition),
}
