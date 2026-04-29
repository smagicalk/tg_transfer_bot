#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoost {
    /// Describes a boost applied to a chat
    #[serde(rename(serialize = "chatBoost", deserialize = "chatBoost"))]
    ChatBoost(crate::types::ChatBoost),
}
