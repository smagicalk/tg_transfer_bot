#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Audio {
    /// Describes an audio file. Audio is usually in MP3 or M4A format
    #[serde(rename(serialize = "audio", deserialize = "audio"))]
    Audio(crate::types::Audio),
}
