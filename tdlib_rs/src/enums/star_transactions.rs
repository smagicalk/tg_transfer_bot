#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarTransactions {
    /// Represents a list of Telegram Star transactions
    #[serde(rename(serialize = "starTransactions", deserialize = "starTransactions"))]
    StarTransactions(crate::types::StarTransactions),
}
