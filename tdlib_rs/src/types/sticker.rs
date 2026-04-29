#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a sticker
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Sticker {
    /// Unique sticker identifier within the set; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the sticker set to which the sticker belongs; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub set_id: i64,
    /// Sticker width; as defined by the sender
    pub width: i32,
    /// Sticker height; as defined by the sender
    pub height: i32,
    /// Emoji corresponding to the sticker; may be empty if unknown
    pub emoji: String,
    /// Sticker format
    pub format: crate::enums::StickerFormat,
    /// Sticker's full type
    pub full_type: crate::enums::StickerFullType,
    /// Sticker thumbnail in WEBP or JPEG format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
    /// File containing the sticker
    pub sticker: crate::types::File,
}
