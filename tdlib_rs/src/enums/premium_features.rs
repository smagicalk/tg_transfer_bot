#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumFeatures {
    /// Contains information about features, available to Premium users
    #[serde(rename(serialize = "premiumFeatures", deserialize = "premiumFeatures"))]
    PremiumFeatures(crate::types::PremiumFeatures),
}
