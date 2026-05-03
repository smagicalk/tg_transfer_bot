#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryArea {
    /// Describes a clickable rectangle area on a story media
    #[serde(rename(serialize = "storyArea", deserialize = "storyArea"))]
    StoryArea(crate::types::StoryArea),
}
