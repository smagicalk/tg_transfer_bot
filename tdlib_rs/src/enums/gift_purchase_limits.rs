#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftPurchaseLimits {
    /// Describes the maximum number of times that a specific gift can be purchased
    #[serde(rename(serialize = "giftPurchaseLimits", deserialize = "giftPurchaseLimits"))]
    GiftPurchaseLimits(crate::types::GiftPurchaseLimits),
}
