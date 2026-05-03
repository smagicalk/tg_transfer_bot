#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumStatePaymentOption {
    /// Describes an option for buying or upgrading Telegram Premium for self
    #[serde(rename(
        serialize = "premiumStatePaymentOption",
        deserialize = "premiumStatePaymentOption"
    ))]
    PremiumStatePaymentOption(crate::types::PremiumStatePaymentOption),
}
