#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Portion of the price of a product (e.g., "delivery cost", "tax amount")
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LabeledPricePart {
    /// Label for this portion of the product price
    pub label: String,
    /// Currency amount in the smallest units of the currency
    pub amount: i64,
}
