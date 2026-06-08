#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// One shipping option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// A list of objects used to calculate the total shipping costs
    pub price_parts: Vec<crate::types::LabeledPricePart>,
}
