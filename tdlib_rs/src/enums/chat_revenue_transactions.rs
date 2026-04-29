#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatRevenueTransactions {
    /// Contains a list of chat revenue transactions
    #[serde(rename(serialize = "chatRevenueTransactions", deserialize = "chatRevenueTransactions"))]
    ChatRevenueTransactions(crate::types::ChatRevenueTransactions),
}
