#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryVideo {
    /// Describes a video file posted as a story
    #[serde(rename(serialize = "storyVideo", deserialize = "storyVideo"))]
    StoryVideo(crate::types::StoryVideo),
}
