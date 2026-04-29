#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A media timestamp
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeMediaTimestamp {
    /// Timestamp from which a video/audio/video note/voice note/story playing must start, in seconds. The media can be in the content or the link preview of the current message, or in the same places in the replied message
    pub media_timestamp: i32,
}
