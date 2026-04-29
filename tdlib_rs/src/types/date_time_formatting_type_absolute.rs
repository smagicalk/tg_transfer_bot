#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The date and time must be shown as absolute timestamps
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DateTimeFormattingTypeAbsolute {
    /// The precision with which hours, minutes and seconds are shown
    pub time_precision: crate::enums::DateTimePartPrecision,
    /// The precision with which the date is shown
    pub date_precision: crate::enums::DateTimePartPrecision,
    /// True, if the day of week must be shown
    pub show_day_of_week: bool,
}
