#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LocationAddress {
    /// Describes an address of a location
    #[serde(rename(serialize = "locationAddress", deserialize = "locationAddress"))]
    LocationAddress(crate::types::LocationAddress),
}
