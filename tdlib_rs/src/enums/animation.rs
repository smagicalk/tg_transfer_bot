#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Animation {
    /// Describes an animation file. The animation must be encoded in GIF or MPEG4 format
    #[serde(rename(serialize = "animation", deserialize = "animation"))]
    Animation(crate::types::Animation),
}
