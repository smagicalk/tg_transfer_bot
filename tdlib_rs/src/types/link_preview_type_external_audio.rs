#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to an audio file
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeExternalAudio {
    /// URL of the audio file
    pub url: String,
    /// MIME type of the audio file
    pub mime_type: String,
    /// Duration of the audio, in seconds; 0 if unknown
    pub duration: i32,
}
