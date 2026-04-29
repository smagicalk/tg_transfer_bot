#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a video player
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeEmbeddedVideoPlayer {
    /// URL of the external video player
    pub url: String,
    /// The cached video; may be null if unknown
    pub video: Option<crate::types::Video>,
    /// Thumbnail of the video; may be null if unknown
    pub thumbnail: Option<crate::types::Photo>,
    /// Duration of the video, in seconds
    pub duration: i32,
    /// Expected width of the embedded player
    pub width: i32,
    /// Expected height of the embedded player
    pub height: i32,
}
