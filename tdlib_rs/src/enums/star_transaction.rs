#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarTransaction {
    /// Represents a transaction changing the amount of owned Telegram Stars
    #[serde(rename(serialize = "starTransaction", deserialize = "starTransaction"))]
    StarTransaction(crate::types::StarTransaction),
}
