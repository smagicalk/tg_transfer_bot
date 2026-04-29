#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a video
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeVideo {
    /// The video description
    pub video: crate::types::Video,
    /// Cover of the video; may be null if none
    pub cover: Option<crate::types::Photo>,
    /// Timestamp from which the video playing must start, in seconds
    pub start_timestamp: i32,
}
