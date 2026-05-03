#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A thumbnail to be sent along with a file; must be in JPEG or WEBP format for stickers, and less than 200 KB in size
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputThumbnail {
    /// Thumbnail file to send. Sending thumbnails by file_id is currently not supported
    pub thumbnail: crate::enums::InputFile,
    /// Thumbnail width, usually shouldn't exceed 320. Use 0 if unknown
    pub width: i32,
    /// Thumbnail height, usually shouldn't exceed 320. Use 0 if unknown
    pub height: i32,
}
