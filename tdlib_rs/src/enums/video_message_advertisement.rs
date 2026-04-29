#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VideoMessageAdvertisement {
    /// Describes an advertisent to be shown while a video from a message is watched
    #[serde(rename(serialize = "videoMessageAdvertisement", deserialize = "videoMessageAdvertisement"))]
    VideoMessageAdvertisement(crate::types::VideoMessageAdvertisement),
}
