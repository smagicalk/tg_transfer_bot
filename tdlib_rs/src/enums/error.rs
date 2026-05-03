#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Error {
    /// An object of this type can be returned on every function call, in case of an error
    #[serde(rename(serialize = "error", deserialize = "error"))]
    Error(crate::types::Error),
}
