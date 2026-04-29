#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A video story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryContentVideo {
    /// The video in MPEG4 format
    pub video: crate::types::StoryVideo,
    /// Alternative version of the video in MPEG4 format, encoded with H.264 codec; may be null
    pub alternative_video: Option<crate::types::StoryVideo>,
}
