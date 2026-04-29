#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VoiceNote {
    /// Describes a voice note
    #[serde(rename(serialize = "voiceNote", deserialize = "voiceNote"))]
    VoiceNote(crate::types::VoiceNote),
}
