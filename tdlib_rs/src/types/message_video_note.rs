#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A video note message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageVideoNote {
    /// The video note description
    pub video_note: crate::types::VideoNote,
    /// True, if at least one of the recipients has viewed the video note
    pub is_viewed: bool,
    /// True, if the video note thumbnail must be blurred and the video note must be shown only while tapped
    pub is_secret: bool,
}
