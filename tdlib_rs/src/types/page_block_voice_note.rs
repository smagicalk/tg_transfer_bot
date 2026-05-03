#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A voice note
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockVoiceNote {
    /// Voice note; may be null
    pub voice_note: Option<crate::types::VoiceNote>,
    /// Voice note caption
    pub caption: crate::types::PageBlockCaption,
}
