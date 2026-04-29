#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageViewers {
    /// Represents a list of message viewers
    #[serde(rename(serialize = "messageViewers", deserialize = "messageViewers"))]
    MessageViewers(crate::types::MessageViewers),
}
