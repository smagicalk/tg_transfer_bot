#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerFormat {
    /// The sticker is an image in WEBP format
    #[serde(rename(serialize = "stickerFormatWebp", deserialize = "stickerFormatWebp"))]
    Webp,
    /// The sticker is an animation in TGS format
    #[serde(rename(serialize = "stickerFormatTgs", deserialize = "stickerFormatTgs"))]
    Tgs,
    /// The sticker is a video in WEBM format
    #[serde(rename(serialize = "stickerFormatWebm", deserialize = "stickerFormatWebm"))]
    Webm,
}
