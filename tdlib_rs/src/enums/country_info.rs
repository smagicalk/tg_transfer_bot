#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CountryInfo {
    /// Contains information about a country
    #[serde(rename(serialize = "countryInfo", deserialize = "countryInfo"))]
    CountryInfo(crate::types::CountryInfo),
}
