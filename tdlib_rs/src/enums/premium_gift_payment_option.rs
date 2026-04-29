#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumGiftPaymentOption {
    /// Describes an option for gifting Telegram Premium to a user. Use telegramPaymentPurposePremiumGift for out-of-store payments or payments in Telegram Stars
    #[serde(rename(serialize = "premiumGiftPaymentOption", deserialize = "premiumGiftPaymentOption"))]
    PremiumGiftPaymentOption(crate::types::PremiumGiftPaymentOption),
}
