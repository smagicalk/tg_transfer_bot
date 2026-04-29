#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumPaymentOption {
    /// Describes an option for buying Telegram Premium to a user
    #[serde(rename(serialize = "premiumPaymentOption", deserialize = "premiumPaymentOption"))]
    PremiumPaymentOption(crate::types::PremiumPaymentOption),
}
