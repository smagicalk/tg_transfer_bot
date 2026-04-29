#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumGiftPaymentOptions {
    /// Contains a list of options for gifting Telegram Premium to a user
    #[serde(rename(serialize = "premiumGiftPaymentOptions", deserialize = "premiumGiftPaymentOptions"))]
    PremiumGiftPaymentOptions(crate::types::PremiumGiftPaymentOptions),
}
