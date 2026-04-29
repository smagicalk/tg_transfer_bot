#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a thumbnail
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Thumbnail {
    /// Thumbnail format
    pub format: crate::enums::ThumbnailFormat,
    /// Thumbnail width
    pub width: i32,
    /// Thumbnail height
    pub height: i32,
    /// The thumbnail
    pub file: crate::types::File,
}
