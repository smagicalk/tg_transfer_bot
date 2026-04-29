#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes the maximum number of times that a specific gift can be purchased
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftPurchaseLimits {
    /// The maximum number of times the gifts can be purchased
    pub total_count: i32,
    /// Number of remaining times the gift can be purchased
    pub remaining_count: i32,
}
