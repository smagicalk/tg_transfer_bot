#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An audio message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentAudio {
    /// Message content; may be null
    pub audio: Option<crate::types::Audio>,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
