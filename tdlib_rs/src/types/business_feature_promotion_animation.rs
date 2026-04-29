#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a promotion animation for a Business feature
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeaturePromotionAnimation {
    /// Business feature
    pub feature: crate::enums::BusinessFeature,
    /// Promotion animation for the feature
    pub animation: crate::types::Animation,
}
