#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSenders {
    /// Represents a list of message senders
    #[serde(rename(serialize = "messageSenders", deserialize = "messageSenders"))]
    MessageSenders(crate::types::MessageSenders),
}
