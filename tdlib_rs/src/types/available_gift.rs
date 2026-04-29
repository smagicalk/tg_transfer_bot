#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a gift that is available for purchase
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AvailableGift {
    /// The gift
    pub gift: crate::types::Gift,
    /// Number of gifts that are available for resale
    pub resale_count: i32,
    /// The minimum price for the gifts available for resale in Telegram Star equivalent; 0 if there are no such gifts
    pub min_resale_star_count: i64,
    /// The title of the upgraded gift; empty if the gift isn't available for resale
    pub title: String,
}
