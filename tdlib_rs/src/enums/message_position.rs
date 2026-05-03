#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessagePosition {
    /// Contains information about a message in a specific position
    #[serde(rename(serialize = "messagePosition", deserialize = "messagePosition"))]
    MessagePosition(crate::types::MessagePosition),
}
