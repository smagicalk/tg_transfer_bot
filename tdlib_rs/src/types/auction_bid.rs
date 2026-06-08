#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a bid in an auction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuctionBid {
    /// The number of Telegram Stars that were put in the bid
    pub star_count: i64,
    /// Point in time (Unix timestamp) when the bid was made
    pub bid_date: i32,
    /// Position of the bid in the list of all bids
    pub position: i32,
}
