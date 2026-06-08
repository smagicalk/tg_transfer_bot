#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A video note message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentVideoNote {
    /// Message content; may be null
    pub video_note: Option<crate::types::VideoNote>,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
