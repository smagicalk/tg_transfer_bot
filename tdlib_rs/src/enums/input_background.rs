#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputBackground {
    /// A background from a local file
    #[serde(rename(serialize = "inputBackgroundLocal", deserialize = "inputBackgroundLocal"))]
    Local(crate::types::InputBackgroundLocal),
    /// A background from the server
    #[serde(rename(serialize = "inputBackgroundRemote", deserialize = "inputBackgroundRemote"))]
    Remote(crate::types::InputBackgroundRemote),
    /// A background previously set in the chat; for chat backgrounds only
    #[serde(rename(serialize = "inputBackgroundPrevious", deserialize = "inputBackgroundPrevious"))]
    Previous(crate::types::InputBackgroundPrevious),
}
