#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A voice note message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageVoiceNote {
    /// The voice note description
    pub voice_note: crate::types::VoiceNote,
    /// Voice note caption
    pub caption: crate::types::FormattedText,
    /// True, if at least one of the recipients has listened to the voice note
    pub is_listened: bool,
}
