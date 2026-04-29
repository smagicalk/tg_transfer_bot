#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A video message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageVideo {
    /// The video description
    pub video: crate::types::Video,
    /// Alternative qualities of the video
    pub alternative_videos: Vec<crate::types::AlternativeVideo>,
    /// Available storyboards for the video
    pub storyboards: Vec<crate::types::VideoStoryboard>,
    /// Cover of the video; may be null if none
    pub cover: Option<crate::types::Photo>,
    /// Timestamp from which the video playing must start, in seconds
    pub start_timestamp: i32,
    /// Video caption
    pub caption: crate::types::FormattedText,
    /// True, if the caption must be shown above the video; otherwise, the caption must be shown below the video
    pub show_caption_above_media: bool,
    /// True, if the video preview must be covered by a spoiler animation
    pub has_spoiler: bool,
    /// True, if the video thumbnail must be blurred and the video must be shown only while tapped
    pub is_secret: bool,
}
