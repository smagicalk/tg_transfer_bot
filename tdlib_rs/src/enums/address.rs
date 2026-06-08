#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Address {
    /// Describes an address
    #[serde(rename(serialize = "address", deserialize = "address"))]
    Address(crate::types::Address),
}
