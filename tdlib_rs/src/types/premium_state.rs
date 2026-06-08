#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains state of Telegram Premium subscription and promotion videos for Premium features
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumState {
    /// Text description of the state of the current Premium subscription; may be empty if the current user has no Telegram Premium subscription
    pub state: crate::types::FormattedText,
    /// The list of available options for buying Telegram Premium
    pub payment_options: Vec<crate::types::PremiumStatePaymentOption>,
    /// The list of available promotion animations for Premium features
    pub animations: Vec<crate::types::PremiumFeaturePromotionAnimation>,
    /// The list of available promotion animations for Business features
    pub business_animations: Vec<crate::types::BusinessFeaturePromotionAnimation>,
}
