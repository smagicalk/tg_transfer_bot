#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostSource {
    /// The chat created a Telegram Premium gift code for a user
    #[serde(rename(serialize = "chatBoostSourceGiftCode", deserialize = "chatBoostSourceGiftCode"))]
    GiftCode(crate::types::ChatBoostSourceGiftCode),
    /// The chat created a giveaway
    #[serde(rename(serialize = "chatBoostSourceGiveaway", deserialize = "chatBoostSourceGiveaway"))]
    Giveaway(crate::types::ChatBoostSourceGiveaway),
    /// A user with Telegram Premium subscription or gifted Telegram Premium boosted the chat
    #[serde(rename(serialize = "chatBoostSourcePremium", deserialize = "chatBoostSourcePremium"))]
    Premium(crate::types::ChatBoostSourcePremium),
}
