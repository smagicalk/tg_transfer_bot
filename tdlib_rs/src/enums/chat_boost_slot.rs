#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostSlot {
    /// Describes a slot for chat boost
    #[serde(rename(serialize = "chatBoostSlot", deserialize = "chatBoostSlot"))]
    ChatBoostSlot(crate::types::ChatBoostSlot),
}
