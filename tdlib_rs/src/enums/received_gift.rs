#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReceivedGift {
    /// Represents a gift received by a user or a chat
    #[serde(rename(serialize = "receivedGift", deserialize = "receivedGift"))]
    ReceivedGift(crate::types::ReceivedGift),
}
