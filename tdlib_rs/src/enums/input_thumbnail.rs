#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputThumbnail {
    /// A thumbnail to be sent along with a file; must be in JPEG or WEBP format for stickers, and less than 200 KB in size
    #[serde(rename(serialize = "inputThumbnail", deserialize = "inputThumbnail"))]
    InputThumbnail(crate::types::InputThumbnail),
}
