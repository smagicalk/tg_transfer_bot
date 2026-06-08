#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarPaymentOption {
    /// Describes an option for buying Telegram Stars. Use telegramPaymentPurposeStars for out-of-store payments
    #[serde(rename(serialize = "starPaymentOption", deserialize = "starPaymentOption"))]
    StarPaymentOption(crate::types::StarPaymentOption),
}
