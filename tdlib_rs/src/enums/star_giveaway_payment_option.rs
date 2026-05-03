#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarGiveawayPaymentOption {
    /// Describes an option for creating of Telegram Star giveaway. Use telegramPaymentPurposeStarGiveaway for out-of-store payments
    #[serde(rename(
        serialize = "starGiveawayPaymentOption",
        deserialize = "starGiveawayPaymentOption"
    ))]
    StarGiveawayPaymentOption(crate::types::StarGiveawayPaymentOption),
}
