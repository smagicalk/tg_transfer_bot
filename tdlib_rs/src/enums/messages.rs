#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Messages {
    /// Contains a list of messages
    #[serde(rename(serialize = "messages", deserialize = "messages"))]
    Messages(crate::types::Messages),
}
