#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VideoMessageAdvertisements {
    /// Contains a list of advertisements to be shown while a video from a message is watched
    #[serde(rename(
        serialize = "videoMessageAdvertisements",
        deserialize = "videoMessageAdvertisements"
    ))]
    VideoMessageAdvertisements(crate::types::VideoMessageAdvertisements),
}
