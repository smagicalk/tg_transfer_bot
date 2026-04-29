#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarGiveawayPaymentOptions {
    /// Contains a list of options for creating of Telegram Star giveaway
    #[serde(rename(serialize = "starGiveawayPaymentOptions", deserialize = "starGiveawayPaymentOptions"))]
    StarGiveawayPaymentOptions(crate::types::StarGiveawayPaymentOptions),
}
