#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TonTransaction {
    /// Represents a transaction changing the amount of owned Toncoins
    #[serde(rename(serialize = "tonTransaction", deserialize = "tonTransaction"))]
    TonTransaction(crate::types::TonTransaction),
}
