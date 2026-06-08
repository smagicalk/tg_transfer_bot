#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of Telegram Star subscriptions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarSubscriptions {
    /// The amount of owned Telegram Stars
    pub star_amount: crate::types::StarAmount,
    /// List of subscriptions for Telegram Stars
    pub subscriptions: Vec<crate::types::StarSubscription>,
    /// The number of Telegram Stars required to buy to extend subscriptions expiring soon
    pub required_star_count: i64,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
