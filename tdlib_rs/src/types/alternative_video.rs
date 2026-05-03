#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an alternative re-encoded quality of a video file
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AlternativeVideo {
    /// Unique identifier of the alternative video, which is used in the HLS file
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Video width
    pub width: i32,
    /// Video height
    pub height: i32,
    /// Codec used for video file encoding, for example, "h264", "h265", "av1", or "av01"
    pub codec: String,
    /// HLS file describing the video
    pub hls_file: crate::types::File,
    /// File containing the video
    pub video: crate::types::File,
}
