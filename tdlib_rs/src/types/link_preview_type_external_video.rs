#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a video file
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeExternalVideo {
    /// URL of the video file
    pub url: String,
    /// MIME type of the video file
    pub mime_type: String,
    /// Expected width of the video preview; 0 if unknown
    pub width: i32,
    /// Expected height of the video preview; 0 if unknown
    pub height: i32,
    /// Duration of the video, in seconds; 0 if unknown
    pub duration: i32,
}
