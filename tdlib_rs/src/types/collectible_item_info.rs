#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a collectible item and its last purchase
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CollectibleItemInfo {
    /// Point in time (Unix timestamp) when the item was purchased
    pub purchase_date: i32,
    /// Currency for the paid amount
    pub currency: String,
    /// The paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Cryptocurrency used to pay for the item
    pub cryptocurrency: String,
    /// The paid amount, in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub cryptocurrency_amount: i64,
    /// Individual URL for the item on https:fragment.com
    pub url: String,
}
