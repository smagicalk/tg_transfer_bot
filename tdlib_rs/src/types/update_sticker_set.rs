#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A sticker set has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateStickerSet {
    /// The sticker set
    pub sticker_set: crate::types::StickerSet,
}
