#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LabeledPricePart {
    /// Portion of the price of a product (e.g., "delivery cost", "tax amount")
    #[serde(rename(serialize = "labeledPricePart", deserialize = "labeledPricePart"))]
    LabeledPricePart(crate::types::LabeledPricePart),
}
