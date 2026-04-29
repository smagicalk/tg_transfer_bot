#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerSet {
    /// Represents a sticker set
    #[serde(rename(serialize = "stickerSet", deserialize = "stickerSet"))]
    StickerSet(crate::types::StickerSet),
}
