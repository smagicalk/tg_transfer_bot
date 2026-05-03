#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatRevenueTransaction {
    /// Contains a chat revenue transactions
    #[serde(rename(
        serialize = "chatRevenueTransaction",
        deserialize = "chatRevenueTransaction"
    ))]
    ChatRevenueTransaction(crate::types::ChatRevenueTransaction),
}
