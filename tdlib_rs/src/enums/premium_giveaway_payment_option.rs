#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumGiveawayPaymentOption {
    /// Describes an option for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members. Use telegramPaymentPurposePremiumGiftCodes or telegramPaymentPurposePremiumGiveaway for out-of-store payments
    #[serde(rename(
        serialize = "premiumGiveawayPaymentOption",
        deserialize = "premiumGiveawayPaymentOption"
    ))]
    PremiumGiveawayPaymentOption(crate::types::PremiumGiveawayPaymentOption),
}
