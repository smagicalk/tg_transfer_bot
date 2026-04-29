#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatEvent {
    /// Represents a chat event
    #[serde(rename(serialize = "chatEvent", deserialize = "chatEvent"))]
    ChatEvent(crate::types::ChatEvent),
}
