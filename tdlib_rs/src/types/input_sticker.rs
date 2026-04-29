#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A sticker to be added to a sticker set
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputSticker {
    /// File with the sticker; must fit in a 512x512 square. For WEBP stickers the file must be in WEBP or PNG format, which will be converted to WEBP server-side.
    /// See https:core.telegram.org/animated_stickers#technical-requirements for technical requirements
    pub sticker: crate::enums::InputFile,
    /// Format of the sticker
    pub format: crate::enums::StickerFormat,
    /// String with 1-20 emoji corresponding to the sticker
    pub emojis: String,
    /// Position where the mask is placed; pass null if not specified
    pub mask_position: Option<crate::types::MaskPosition>,
    /// List of up to 20 keywords with total length up to 64 characters, which can be used to find the sticker
    pub keywords: Vec<String>,
}
