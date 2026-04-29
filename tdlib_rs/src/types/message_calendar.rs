#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about found messages, split by days according to the option "utc_time_offset"
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageCalendar {
    /// Total number of found messages
    pub total_count: i32,
    /// Information about messages sent
    pub days: Vec<crate::types::MessageCalendarDay>,
}
