#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TrendingStickerSets {
    /// Represents a list of trending sticker sets
    #[serde(rename(serialize = "trendingStickerSets", deserialize = "trendingStickerSets"))]
    TrendingStickerSets(crate::types::TrendingStickerSets),
}
