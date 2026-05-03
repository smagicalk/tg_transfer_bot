#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of chat revenue transactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransactions {
    /// The amount of owned Toncoins; in the smallest units of the cryptocurrency
    pub ton_amount: i64,
    /// List of transactions
    pub transactions: Vec<crate::types::ChatRevenueTransaction>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
