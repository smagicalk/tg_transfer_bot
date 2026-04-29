#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a time zone
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TimeZone {
    /// Unique time zone identifier
    pub id: String,
    /// Time zone name
    pub name: String,
    /// Current UTC time offset for the time zone
    pub utc_time_offset: i32,
}
