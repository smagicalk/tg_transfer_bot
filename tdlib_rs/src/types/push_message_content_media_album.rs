#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A media album
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentMediaAlbum {
    /// Number of messages in the album
    pub total_count: i32,
    /// True, if the album has at least one photo
    pub has_photos: bool,
    /// True, if the album has at least one video file
    pub has_videos: bool,
    /// True, if the album has at least one audio file
    pub has_audios: bool,
    /// True, if the album has at least one document
    pub has_documents: bool,
}
