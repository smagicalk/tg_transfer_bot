#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of advertisements to be shown while a video from a message is watched
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VideoMessageAdvertisements {
    /// List of advertisements
    pub advertisements: Vec<crate::types::VideoMessageAdvertisement>,
    /// Delay before the first advertisement is shown, in seconds
    pub start_delay: i32,
    /// Delay between consecutive advertisements, in seconds
    pub between_delay: i32,
}
