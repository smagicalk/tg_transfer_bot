#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a video note. The video must be equal in width and height, cropped to a circle, and stored in MPEG4 format
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VideoNote {
    /// Duration of the video, in seconds; as defined by the sender
    pub duration: i32,
    /// A waveform representation of the video note's audio in 5-bit format; may be empty if unknown
    pub waveform: String,
    /// Video width and height; as defined by the sender
    pub length: i32,
    /// Video minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
    /// Video thumbnail in JPEG format; as defined by the sender; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
    /// Result of speech recognition in the video note; may be null
    pub speech_recognition_result: Option<crate::enums::SpeechRecognitionResult>,
    /// File containing the video
    pub video: crate::types::File,
}
