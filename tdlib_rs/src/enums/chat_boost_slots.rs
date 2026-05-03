#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostSlots {
    /// Contains a list of chat boost slots
    #[serde(rename(serialize = "chatBoostSlots", deserialize = "chatBoostSlots"))]
    ChatBoostSlots(crate::types::ChatBoostSlots),
}
