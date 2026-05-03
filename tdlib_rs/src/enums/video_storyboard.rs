#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VideoStoryboard {
    /// Describes a storyboard for a video
    #[serde(rename(serialize = "videoStoryboard", deserialize = "videoStoryboard"))]
    VideoStoryboard(crate::types::VideoStoryboard),
}
