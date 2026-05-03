#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an auction on which a gift can be purchased
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftAuction {
    /// Identifier of the auction
    pub id: String,
    /// Number of gifts distributed in each round
    pub gifts_per_round: i32,
    /// Point in time (Unix timestamp) when the auction will start
    pub start_date: i32,
}
