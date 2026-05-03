#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of sticker sets
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerSets {
    /// Approximate total number of sticker sets found
    pub total_count: i32,
    /// List of sticker sets
    pub sets: Vec<crate::types::StickerSetInfo>,
}
