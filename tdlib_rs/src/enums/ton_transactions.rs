#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TonTransactions {
    /// Represents a list of Toncoin transactions
    #[serde(rename(serialize = "tonTransactions", deserialize = "tonTransactions"))]
    TonTransactions(crate::types::TonTransactions),
}
