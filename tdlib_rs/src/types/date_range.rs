#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a date range
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DateRange {
    /// Point in time (Unix timestamp) at which the date range begins
    pub start_date: i32,
    /// Point in time (Unix timestamp) at which the date range ends
    pub end_date: i32,
}
