#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Send away messages only in the specified time span
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessAwayMessageScheduleCustom {
    /// Point in time (Unix timestamp) when the away messages will start to be sent
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the away messages will stop to be sent
    pub end_date: i32,
}
