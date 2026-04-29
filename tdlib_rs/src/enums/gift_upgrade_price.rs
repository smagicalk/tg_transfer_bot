#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftUpgradePrice {
    /// Describes a price required to pay to upgrade a gift
    #[serde(rename(serialize = "giftUpgradePrice", deserialize = "giftUpgradePrice"))]
    GiftUpgradePrice(crate::types::GiftUpgradePrice),
}
