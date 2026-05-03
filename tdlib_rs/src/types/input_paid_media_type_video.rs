#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The media is a video
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPaidMediaTypeVideo {
    /// Cover of the video; pass null to skip cover uploading
    pub cover: Option<crate::enums::InputFile>,
    /// Timestamp from which the video playing must start, in seconds
    pub start_timestamp: i32,
    /// Duration of the video, in seconds
    pub duration: i32,
    /// True, if the video is expected to be streamed
    pub supports_streaming: bool,
}
