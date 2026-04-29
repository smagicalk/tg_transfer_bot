#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessagePositions {
    /// Contains a list of message positions
    #[serde(rename(serialize = "messagePositions", deserialize = "messagePositions"))]
    MessagePositions(crate::types::MessagePositions),
}
