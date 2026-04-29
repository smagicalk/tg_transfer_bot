#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes opening hours of a business
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessOpeningHours {
    /// Unique time zone identifier
    pub time_zone_id: String,
    /// Intervals of the time when the business is open
    pub opening_hours: Vec<crate::types::BusinessOpeningHoursInterval>,
}
