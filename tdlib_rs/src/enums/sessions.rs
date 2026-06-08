#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Sessions {
    /// Contains a list of sessions
    #[serde(rename(serialize = "sessions", deserialize = "sessions"))]
    Sessions(crate::types::Sessions),
}
