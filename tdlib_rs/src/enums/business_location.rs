#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessLocation {
    /// Represents a location of a business
    #[serde(rename(serialize = "businessLocation", deserialize = "businessLocation"))]
    BusinessLocation(crate::types::BusinessLocation),
}
