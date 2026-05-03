#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a sticker set
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StickerSet {
    /// Identifier of the sticker set
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Title of the sticker set
    pub title: String,
    /// Name of the sticker set
    pub name: String,
    /// Sticker set thumbnail in WEBP, TGS, or WEBM format with width and height 100; may be null. The file can be downloaded only before the thumbnail is changed
    pub thumbnail: Option<crate::types::Thumbnail>,
    /// Sticker set thumbnail's outline; may be null if unknown
    pub thumbnail_outline: Option<crate::types::Outline>,
    /// True, if the sticker set is owned by the current user
    pub is_owned: bool,
    /// True, if the sticker set has been installed by the current user
    pub is_installed: bool,
    /// True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously
    pub is_archived: bool,
    /// True, if the sticker set is official
    pub is_official: bool,
    /// Type of the stickers in the set
    pub sticker_type: crate::enums::StickerType,
    /// True, if stickers in the sticker set are custom emoji that must be repainted; for custom emoji sticker sets only
    pub needs_repainting: bool,
    /// True, if stickers in the sticker set are custom emoji that can be used as chat emoji status; for custom emoji sticker sets only
    pub is_allowed_as_chat_emoji_status: bool,
    /// True for already viewed trending sticker sets
    pub is_viewed: bool,
    /// List of stickers in this set
    pub stickers: Vec<crate::types::Sticker>,
    /// A list of emojis corresponding to the stickers in the same order. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
    pub emojis: Vec<crate::types::Emojis>,
}
