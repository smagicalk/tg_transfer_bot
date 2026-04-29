#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of trending sticker sets was updated or some of them were viewed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateTrendingStickerSets {
    /// Type of the affected stickers
    pub sticker_type: crate::enums::StickerType,
    /// The prefix of the list of trending sticker sets with the newest trending sticker sets
    pub sticker_sets: crate::types::TrendingStickerSets,
}
