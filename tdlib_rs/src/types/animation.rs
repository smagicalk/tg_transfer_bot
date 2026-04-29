#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes an animation file. The animation must be encoded in GIF or MPEG4 format
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Animation {
    /// Duration of the animation, in seconds; as defined by the sender
    pub duration: i32,
    /// Width of the animation
    pub width: i32,
    /// Height of the animation
    pub height: i32,
    /// Original name of the file; as defined by the sender
    pub file_name: String,
    /// MIME type of the file, usually "image/gif" or "video/mp4"
    pub mime_type: String,
    /// True, if stickers were added to the animation. The list of corresponding sticker set can be received using getAttachedStickerSets
    pub has_stickers: bool,
    /// Animation minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
    /// Animation thumbnail in JPEG or MPEG4 format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
    /// File containing the animation
    pub animation: crate::types::File,
}
