#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerType {
    /// The sticker is a regular sticker
    #[serde(rename(serialize = "stickerTypeRegular", deserialize = "stickerTypeRegular"))]
    Regular,
    /// The sticker is a mask in WEBP format to be placed on photos or videos
    #[serde(rename(serialize = "stickerTypeMask", deserialize = "stickerTypeMask"))]
    Mask,
    /// The sticker is a custom emoji to be used inside message text and caption
    #[serde(rename(
        serialize = "stickerTypeCustomEmoji",
        deserialize = "stickerTypeCustomEmoji"
    ))]
    CustomEmoji,
}
