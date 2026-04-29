#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of Toncoin transactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TonTransactions {
    /// The total amount of owned Toncoins
    pub ton_amount: i64,
    /// List of Toncoin transactions
    pub transactions: Vec<crate::types::TonTransaction>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
