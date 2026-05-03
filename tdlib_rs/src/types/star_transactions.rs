#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of Telegram Star transactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactions {
    /// The amount of owned Telegram Stars
    pub star_amount: crate::types::StarAmount,
    /// List of transactions with Telegram Stars
    pub transactions: Vec<crate::types::StarTransaction>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
