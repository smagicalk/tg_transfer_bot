#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A voice note message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentVoiceNote {
    /// Message content; may be null
    pub voice_note: Option<crate::types::VoiceNote>,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
