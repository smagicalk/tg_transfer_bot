#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftUpgradeVariants {
    /// Contains all possible variants of upgraded gifts for the given regular gift
    #[serde(rename(serialize = "giftUpgradeVariants", deserialize = "giftUpgradeVariants"))]
    GiftUpgradeVariants(crate::types::GiftUpgradeVariants),
}
