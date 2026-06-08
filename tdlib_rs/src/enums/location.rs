#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Location {
    /// Describes a location on planet Earth
    #[serde(rename(serialize = "location", deserialize = "location"))]
    Location(crate::types::Location),
}
