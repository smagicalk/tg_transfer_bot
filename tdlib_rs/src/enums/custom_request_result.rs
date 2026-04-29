#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CustomRequestResult {
    /// Contains the result of a custom request
    #[serde(rename(serialize = "customRequestResult", deserialize = "customRequestResult"))]
    CustomRequestResult(crate::types::CustomRequestResult),
}
