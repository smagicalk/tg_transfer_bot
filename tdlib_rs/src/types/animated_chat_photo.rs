#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Animated variant of a chat photo in MPEG4 format
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AnimatedChatPhoto {
    /// Animation width and height
    pub length: i32,
    /// Information about the animation file
    pub file: crate::types::File,
    /// Timestamp of the frame, used as a static chat photo
    pub main_frame_timestamp: f64,
}
