#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerSetInfo {
    /// Represents short information about a sticker set
    #[serde(rename(serialize = "stickerSetInfo", deserialize = "stickerSetInfo"))]
    StickerSetInfo(crate::types::StickerSetInfo),
}
