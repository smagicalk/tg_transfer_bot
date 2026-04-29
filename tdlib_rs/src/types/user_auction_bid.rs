#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a bid of the current user in an auction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserAuctionBid {
    /// The number of Telegram Stars that were put in the bid
    pub star_count: i64,
    /// Point in time (Unix timestamp) when the bid was made
    pub bid_date: i32,
    /// The minimum number of Telegram Stars that can be put for the next bid
    pub next_bid_star_count: i64,
    /// Identifier of the user or the chat that will receive the auctioned item. If the auction is opened in context of another user or chat, then a warning is supposed to be shown to the current user
    pub owner_id: crate::enums::MessageSender,
    /// True, if the bid was returned to the user, because it was outbid and can't win anymore
    pub was_returned: bool,
}
