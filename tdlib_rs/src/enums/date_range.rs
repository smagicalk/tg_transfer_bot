#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DateRange {
    /// Represents a date range
    #[serde(rename(serialize = "dateRange", deserialize = "dateRange"))]
    DateRange(crate::types::DateRange),
}
