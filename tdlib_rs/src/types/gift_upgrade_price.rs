#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a price required to pay to upgrade a gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftUpgradePrice {
    /// Point in time (Unix timestamp) when the price will be in effect
    pub date: i32,
    /// The Telegram Star amount required to pay to upgrade the gift
    pub star_count: i64,
}
