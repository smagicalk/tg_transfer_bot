#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumGiveawayPaymentOptions {
    /// Contains a list of options for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members
    #[serde(rename(serialize = "premiumGiveawayPaymentOptions", deserialize = "premiumGiveawayPaymentOptions"))]
    PremiumGiveawayPaymentOptions(crate::types::PremiumGiveawayPaymentOptions),
}
