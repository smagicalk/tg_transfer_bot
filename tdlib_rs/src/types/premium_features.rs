#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about features, available to Premium users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatures {
    /// The list of available features
    pub features: Vec<crate::enums::PremiumFeature>,
    /// The list of limits, increased for Premium users
    pub limits: Vec<crate::types::PremiumLimit>,
    /// An internal link to be opened to pay for Telegram Premium if store payment isn't possible; may be null if direct payment isn't available
    pub payment_link: Option<crate::enums::InternalLinkType>,
}
