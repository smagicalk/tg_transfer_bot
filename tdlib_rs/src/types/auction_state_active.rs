#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about an ongoing or scheduled auction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AuctionStateActive {
    /// Point in time (Unix timestamp) when the auction started or will start
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the auction will be ended
    pub end_date: i32,
    /// The minimum possible bid in the auction in Telegram Stars
    pub min_bid: i64,
    /// A sparse list of bids that were made in the auction
    pub bid_levels: Vec<crate::types::AuctionBid>,
    /// User identifiers of at most 3 users with the biggest bids
    pub top_bidder_user_ids: Vec<i64>,
    /// Rounds of the auction in which their duration or extension rules are changed
    pub rounds: Vec<crate::types::AuctionRound>,
    /// Point in time (Unix timestamp) when the current round will end
    pub current_round_end_date: i32,
    /// 1-based number of the current round
    pub current_round_number: i32,
    /// The total number of rounds
    pub total_round_count: i32,
    /// The number of items that were purchased on the auction by all users
    pub distributed_item_count: i32,
    /// The number of items that have to be distributed on the auction
    pub left_item_count: i32,
    /// The number of items that were purchased by the current user on the auction
    pub acquired_item_count: i32,
    /// Bid of the current user in the auction; may be null if none
    pub user_bid: Option<crate::types::UserAuctionBid>,
}
