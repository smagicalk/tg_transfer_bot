#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallMessage {
    /// Represents a message sent in a group call
    #[serde(rename(serialize = "groupCallMessage", deserialize = "groupCallMessage"))]
    GroupCallMessage(crate::types::GroupCallMessage),
}
