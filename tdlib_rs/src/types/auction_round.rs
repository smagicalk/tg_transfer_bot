#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a round of an auction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuctionRound {
    /// 1-based number of the round
    pub number: i32,
    /// Duration of the round, in seconds
    pub duration: i32,
    /// The number of seconds for which the round will be extended if there are changes in the top winners
    pub extend_time: i32,
    /// The number of top winners who trigger round extension if changed
    pub top_winner_count: i32,
}
