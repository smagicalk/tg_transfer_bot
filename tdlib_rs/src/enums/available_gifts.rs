#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AvailableGifts {
    /// Contains a list of gifts that can be sent to another user or channel chat
    #[serde(rename(serialize = "availableGifts", deserialize = "availableGifts"))]
    AvailableGifts(crate::types::AvailableGifts),
}
