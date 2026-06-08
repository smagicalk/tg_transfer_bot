#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumFeaturePromotionAnimation {
    /// Describes a promotion animation for a Premium feature
    #[serde(rename(
        serialize = "premiumFeaturePromotionAnimation",
        deserialize = "premiumFeaturePromotionAnimation"
    ))]
    PremiumFeaturePromotionAnimation(crate::types::PremiumFeaturePromotionAnimation),
}
