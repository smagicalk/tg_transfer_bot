#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallId {
    /// Contains the call identifier
    #[serde(rename(serialize = "callId", deserialize = "callId"))]
    CallId(crate::types::CallId),
}
