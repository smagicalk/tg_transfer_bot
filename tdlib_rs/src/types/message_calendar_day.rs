#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about found messages sent on a specific day
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageCalendarDay {
    /// Total number of found messages sent on the day
    pub total_count: i32,
    /// First message sent on the day
    pub message: crate::types::Message,
}
