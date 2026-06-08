#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TimeZones {
    /// Contains a list of time zones
    #[serde(rename(serialize = "timeZones", deserialize = "timeZones"))]
    TimeZones(crate::types::TimeZones),
}
