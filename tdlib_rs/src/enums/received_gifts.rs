#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReceivedGifts {
    /// Represents a list of gifts received by a user or a chat
    #[serde(rename(serialize = "receivedGifts", deserialize = "receivedGifts"))]
    ReceivedGifts(crate::types::ReceivedGifts),
}
