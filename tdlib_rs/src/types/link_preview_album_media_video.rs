#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The media is a video
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewAlbumMediaVideo {
    /// Video description
    pub video: crate::types::Video,
}
