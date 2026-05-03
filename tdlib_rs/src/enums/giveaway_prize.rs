#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiveawayPrize {
    /// The giveaway sends Telegram Premium subscriptions to the winners
    #[serde(rename(
        serialize = "giveawayPrizePremium",
        deserialize = "giveawayPrizePremium"
    ))]
    Premium(crate::types::GiveawayPrizePremium),
    /// The giveaway sends Telegram Stars to the winners
    #[serde(rename(serialize = "giveawayPrizeStars", deserialize = "giveawayPrizeStars"))]
    Stars(crate::types::GiveawayPrizeStars),
}
