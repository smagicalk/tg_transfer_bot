#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatEvents {
    /// Contains a list of chat events
    #[serde(rename(serialize = "chatEvents", deserialize = "chatEvents"))]
    ChatEvents(crate::types::ChatEvents),
}
