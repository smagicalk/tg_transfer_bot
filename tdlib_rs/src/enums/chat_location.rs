#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatLocation {
    /// Represents a location to which a chat is connected
    #[serde(rename(serialize = "chatLocation", deserialize = "chatLocation"))]
    ChatLocation(crate::types::ChatLocation),
}
