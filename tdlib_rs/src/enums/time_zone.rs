#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TimeZone {
    /// Describes a time zone
    #[serde(rename(serialize = "timeZone", deserialize = "timeZone"))]
    TimeZone(crate::types::TimeZone),
}
