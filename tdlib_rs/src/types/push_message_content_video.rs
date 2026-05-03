#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A video message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentVideo {
    /// Message content; may be null
    pub video: Option<crate::types::Video>,
    /// Video caption
    pub caption: String,
    /// True, if the video is secret
    pub is_secret: bool,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
