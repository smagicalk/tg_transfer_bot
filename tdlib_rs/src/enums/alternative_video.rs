#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AlternativeVideo {
    /// Describes an alternative re-encoded quality of a video file
    #[serde(rename(serialize = "alternativeVideo", deserialize = "alternativeVideo"))]
    AlternativeVideo(crate::types::AlternativeVideo),
}
