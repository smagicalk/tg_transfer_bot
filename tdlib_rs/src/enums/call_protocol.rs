#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallProtocol {
    /// Specifies the supported call protocols
    #[serde(rename(serialize = "callProtocol", deserialize = "callProtocol"))]
    CallProtocol(crate::types::CallProtocol),
}
