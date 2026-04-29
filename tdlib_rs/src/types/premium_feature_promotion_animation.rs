#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a promotion animation for a Premium feature
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeaturePromotionAnimation {
    /// Premium feature
    pub feature: crate::enums::PremiumFeature,
    /// Promotion animation for the feature
    pub animation: crate::types::Animation,
}
