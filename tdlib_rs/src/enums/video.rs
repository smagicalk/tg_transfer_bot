#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Video {
    /// Describes a video file
    #[serde(rename(serialize = "video", deserialize = "video"))]
    Video(crate::types::Video),
}
