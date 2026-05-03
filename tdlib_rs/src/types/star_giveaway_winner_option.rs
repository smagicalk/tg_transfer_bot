#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an option for the number of winners of a Telegram Star giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarGiveawayWinnerOption {
    /// The number of users that will be chosen as winners
    pub winner_count: i32,
    /// The number of Telegram Stars that will be won by the winners of the giveaway
    pub won_star_count: i64,
    /// True, if the option must be chosen by default
    pub is_default: bool,
}
