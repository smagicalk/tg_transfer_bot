#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes earnings from sponsored messages in a chat in some time frame
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransactionTypeSponsoredMessageEarnings {
    /// Point in time (Unix timestamp) when the earnings started
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the earnings ended
    pub end_date: i32,
}
