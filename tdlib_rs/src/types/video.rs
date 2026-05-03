#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a video file
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Video {
    /// Duration of the video, in seconds; as defined by the sender
    pub duration: i32,
    /// Video width; as defined by the sender
    pub width: i32,
    /// Video height; as defined by the sender
    pub height: i32,
    /// Original name of the file; as defined by the sender
    pub file_name: String,
    /// MIME type of the file; as defined by the sender
    pub mime_type: String,
    /// True, if stickers were added to the video. The list of corresponding sticker sets can be received using getAttachedStickerSets
    pub has_stickers: bool,
    /// True, if the video is expected to be streamed
    pub supports_streaming: bool,
    /// Video minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
    /// Video thumbnail in JPEG or MPEG4 format; as defined by the sender; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
    /// File containing the video
    pub video: crate::types::File,
}
