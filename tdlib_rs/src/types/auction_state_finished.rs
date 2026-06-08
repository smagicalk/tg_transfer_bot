#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a finished auction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuctionStateFinished {
    /// Point in time (Unix timestamp) when the auction started
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the auction will be ended
    pub end_date: i32,
    /// Average price of bought items in Telegram Stars
    pub average_price: i64,
    /// The number of items that were purchased by the current user on the auction
    pub acquired_item_count: i32,
    /// Number of items from the auction being resold on Telegram
    pub telegram_listed_item_count: i32,
    /// Number of items from the auction being resold on Fragment
    pub fragment_listed_item_count: i32,
    /// The HTTPS link to the Fragment for the resold items; may be empty if there are no such items being sold on Fragment
    pub fragment_url: String,
}
