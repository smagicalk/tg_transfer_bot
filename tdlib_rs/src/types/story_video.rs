#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a video file posted as a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryVideo {
    /// Duration of the video, in seconds
    pub duration: f64,
    /// Video width
    pub width: i32,
    /// Video height
    pub height: i32,
    /// True, if stickers were added to the video. The list of corresponding sticker sets can be received using getAttachedStickerSets
    pub has_stickers: bool,
    /// True, if the video has no sound
    pub is_animation: bool,
    /// Video minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
    /// Video thumbnail in JPEG or MPEG4 format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
    /// Size of file prefix, which is expected to be preloaded, in bytes
    pub preload_prefix_size: i32,
    /// Timestamp of the frame used as video thumbnail
    pub cover_frame_timestamp: f64,
    /// File containing the video
    pub video: crate::types::File,
}
