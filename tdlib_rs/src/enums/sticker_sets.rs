#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerSets {
    /// Represents a list of sticker sets
    #[serde(rename(serialize = "stickerSets", deserialize = "stickerSets"))]
    StickerSets(crate::types::StickerSets),
}
