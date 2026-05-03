#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageProperties {
    /// Contains properties of a message and describes actions that can be done with the message right now
    #[serde(rename(serialize = "messageProperties", deserialize = "messageProperties"))]
    MessageProperties(crate::types::MessageProperties),
}
