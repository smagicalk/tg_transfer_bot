#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatRevenueAmount {
    /// Contains information about revenue earned from sponsored messages in a chat
    #[serde(rename(serialize = "chatRevenueAmount", deserialize = "chatRevenueAmount"))]
    ChatRevenueAmount(crate::types::ChatRevenueAmount),
}
