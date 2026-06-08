#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TransactionDirection {
    /// The transaction is incoming and increases the amount of owned currency
    #[serde(rename(
        serialize = "transactionDirectionIncoming",
        deserialize = "transactionDirectionIncoming"
    ))]
    Incoming,
    /// The transaction is outgoing and decreases the amount of owned currency
    #[serde(rename(
        serialize = "transactionDirectionOutgoing",
        deserialize = "transactionDirectionOutgoing"
    ))]
    Outgoing,
}
