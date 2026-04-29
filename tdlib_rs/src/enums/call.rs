#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Call {
    /// Describes a call
    #[serde(rename(serialize = "call", deserialize = "call"))]
    Call(crate::types::Call),
}
