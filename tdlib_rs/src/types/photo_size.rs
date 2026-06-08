#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an image in JPEG format
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PhotoSize {
    /// Image type (see https:core.telegram.org/constructor/photoSize)
    pub r#type: String,
    /// Information about the image file
    pub photo: crate::types::File,
    /// Image width
    pub width: i32,
    /// Image height
    pub height: i32,
    /// Sizes of progressive JPEG file prefixes, which can be used to preliminarily show the image; in bytes
    pub progressive_sizes: Vec<i32>,
}
