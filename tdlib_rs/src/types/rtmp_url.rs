#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents an RTMP URL
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RtmpUrl {
    /// The URL
    pub url: String,
    /// Stream key
    pub stream_key: String,
}
