#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A video story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputStoryContentVideo {
    /// Video to be sent. The video size must be 720x1280. The video must be streamable and stored in MPEG4 format, after encoding with H.265 codec and key frames added each second
    pub video: crate::enums::InputFile,
    /// File identifiers of the stickers added to the video, if applicable
    pub added_sticker_file_ids: Vec<i32>,
    /// Precise duration of the video, in seconds; 0-60
    pub duration: f64,
    /// Timestamp of the frame, which will be used as video thumbnail
    pub cover_frame_timestamp: f64,
    /// True, if the video has no sound
    pub is_animation: bool,
}
