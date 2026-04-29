#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to an audio player
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeEmbeddedAudioPlayer {
    /// URL of the external audio player
    pub url: String,
    /// The cached audio; may be null if unknown
    pub audio: Option<crate::types::Audio>,
    /// Thumbnail of the audio; may be null if unknown
    pub thumbnail: Option<crate::types::Photo>,
    /// Duration of the audio, in seconds
    pub duration: i32,
    /// Expected width of the embedded player
    pub width: i32,
    /// Expected height of the embedded player
    pub height: i32,
}
