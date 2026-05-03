#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about public post search limits
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PublicPostSearchLimits {
    /// Number of queries that can be sent daily for free
    pub daily_free_query_count: i32,
    /// Number of remaining free queries today
    pub remaining_free_query_count: i32,
    /// Amount of time till the next free query can be sent; 0 if it can be sent now
    pub next_free_query_in: i32,
    /// Number of Telegram Stars that must be paid for each non-free query
    pub star_count: i64,
    /// True, if the search for the specified query isn't charged
    pub is_current_query_free: bool,
}
