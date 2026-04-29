#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallServer {
    /// Describes a server for relaying call data
    #[serde(rename(serialize = "callServer", deserialize = "callServer"))]
    CallServer(crate::types::CallServer),
}
