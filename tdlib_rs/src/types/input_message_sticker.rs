#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A sticker message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageSticker {
    /// Sticker to be sent
    pub sticker: crate::enums::InputFile,
    /// Sticker thumbnail; pass null to skip thumbnail uploading
    pub thumbnail: Option<crate::types::InputThumbnail>,
    /// Sticker width
    pub width: i32,
    /// Sticker height
    pub height: i32,
    /// Emoji used to choose the sticker
    pub emoji: String,
}
