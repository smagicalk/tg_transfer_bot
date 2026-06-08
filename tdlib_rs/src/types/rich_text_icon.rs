#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A small image inside the text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextIcon {
    /// The image represented as a document. The image can be in GIF, JPEG or PNG format
    pub document: crate::types::Document,
    /// Width of a bounding box in which the image must be shown; 0 if unknown
    pub width: i32,
    /// Height of a bounding box in which the image must be shown; 0 if unknown
    pub height: i32,
}
