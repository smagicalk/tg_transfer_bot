#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a voice note
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VoiceNote {
    /// Duration of the voice note, in seconds; as defined by the sender
    pub duration: i32,
    /// A waveform representation of the voice note in 5-bit format
    pub waveform: String,
    /// MIME type of the file; as defined by the sender. Usually, one of "audio/ogg" for Opus in an OGG container, "audio/mpeg" for an MP3 audio, or "audio/mp4" for an M4A audio
    pub mime_type: String,
    /// Result of speech recognition in the voice note; may be null
    pub speech_recognition_result: Option<crate::enums::SpeechRecognitionResult>,
    /// File containing the voice note
    pub voice: crate::types::File,
}
