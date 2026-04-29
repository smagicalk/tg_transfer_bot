#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VideoNote {
    /// Describes a video note. The video must be equal in width and height, cropped to a circle, and stored in MPEG4 format
    #[serde(rename(serialize = "videoNote", deserialize = "videoNote"))]
    VideoNote(crate::types::VideoNote),
}
