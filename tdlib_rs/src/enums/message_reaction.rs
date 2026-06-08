#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageReaction {
    /// Contains information about a reaction to a message
    #[serde(rename(serialize = "messageReaction", deserialize = "messageReaction"))]
    MessageReaction(crate::types::MessageReaction),
}
