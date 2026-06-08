#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RtmpUrl {
    /// Represents an RTMP URL
    #[serde(rename(serialize = "rtmpUrl", deserialize = "rtmpUrl"))]
    RtmpUrl(crate::types::RtmpUrl),
}
