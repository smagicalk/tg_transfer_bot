#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerFullType {
    /// The sticker is a regular sticker
    #[serde(rename(serialize = "stickerFullTypeRegular", deserialize = "stickerFullTypeRegular"))]
    Regular(crate::types::StickerFullTypeRegular),
    /// The sticker is a mask in WEBP format to be placed on photos or videos
    #[serde(rename(serialize = "stickerFullTypeMask", deserialize = "stickerFullTypeMask"))]
    Mask(crate::types::StickerFullTypeMask),
    /// The sticker is a custom emoji to be used inside message text and caption. Currently, only Telegram Premium users can use custom emoji
    #[serde(rename(serialize = "stickerFullTypeCustomEmoji", deserialize = "stickerFullTypeCustomEmoji"))]
    CustomEmoji(crate::types::StickerFullTypeCustomEmoji),
}
