#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallDataChannel {
    /// The main data channel for audio and video data
    #[serde(rename(serialize = "groupCallDataChannelMain", deserialize = "groupCallDataChannelMain"))]
    Main,
    /// The data channel for screen sharing
    #[serde(rename(serialize = "groupCallDataChannelScreenSharing", deserialize = "groupCallDataChannelScreenSharing"))]
    ScreenSharing,
}
