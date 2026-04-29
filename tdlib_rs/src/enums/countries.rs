#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Countries {
    /// Contains information about countries
    #[serde(rename(serialize = "countries", deserialize = "countries"))]
    Countries(crate::types::Countries),
}
