#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessFeaturePromotionAnimation {
    /// Describes a promotion animation for a Business feature
    #[serde(rename(
        serialize = "businessFeaturePromotionAnimation",
        deserialize = "businessFeaturePromotionAnimation"
    ))]
    BusinessFeaturePromotionAnimation(crate::types::BusinessFeaturePromotionAnimation),
}
