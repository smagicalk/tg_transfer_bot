#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftUpgradePreview {
    /// Contains examples of possible upgraded gifts for the given regular gift
    #[serde(rename(serialize = "giftUpgradePreview", deserialize = "giftUpgradePreview"))]
    GiftUpgradePreview(crate::types::GiftUpgradePreview),
}
