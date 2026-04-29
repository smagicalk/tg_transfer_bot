#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageReactions {
    /// Contains a list of reactions added to a message
    #[serde(rename(serialize = "messageReactions", deserialize = "messageReactions"))]
    MessageReactions(crate::types::MessageReactions),
}
