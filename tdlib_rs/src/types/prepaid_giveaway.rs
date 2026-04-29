#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a prepaid giveaway
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PrepaidGiveaway {
    /// Unique identifier of the prepaid giveaway
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Number of users which will receive giveaway prize
    pub winner_count: i32,
    /// Prize of the giveaway
    pub prize: crate::enums::GiveawayPrize,
    /// The number of boosts received by the chat from the giveaway; for Telegram Star giveaways only
    pub boost_count: i32,
    /// Point in time (Unix timestamp) when the giveaway was paid
    pub payment_date: i32,
}
