#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AvailableGift {
    /// Describes a gift that is available for purchase
    #[serde(rename(serialize = "availableGift", deserialize = "availableGift"))]
    AvailableGift(crate::types::AvailableGift),
}
