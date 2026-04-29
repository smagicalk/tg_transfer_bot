#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Message {
    /// Describes a message
    #[serde(rename(serialize = "message", deserialize = "message"))]
    Message(crate::types::Message),
}
