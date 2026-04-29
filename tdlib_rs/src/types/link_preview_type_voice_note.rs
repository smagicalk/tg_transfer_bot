#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a voice note message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeVoiceNote {
    /// The voice note
    pub voice_note: crate::types::VoiceNote,
}
