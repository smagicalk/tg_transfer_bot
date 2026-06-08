#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of trending sticker sets
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TrendingStickerSets {
    /// Approximate total number of trending sticker sets
    pub total_count: i32,
    /// List of trending sticker sets
    pub sets: Vec<crate::types::StickerSetInfo>,
    /// True, if the list contains sticker sets with premium stickers
    pub is_premium: bool,
}
