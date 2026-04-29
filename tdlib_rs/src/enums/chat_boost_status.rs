#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostStatus {
    /// Describes current boost status of a chat
    #[serde(rename(serialize = "chatBoostStatus", deserialize = "chatBoostStatus"))]
    ChatBoostStatus(crate::types::ChatBoostStatus),
}
