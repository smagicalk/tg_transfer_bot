#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Thumbnail image of a very poor quality and low resolution
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Minithumbnail {
    /// Thumbnail width, usually doesn't exceed 40
    pub width: i32,
    /// Thumbnail height, usually doesn't exceed 40
    pub height: i32,
    /// The thumbnail in JPEG format
    pub data: String,
}
