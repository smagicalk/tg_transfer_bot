#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a date according to the Gregorian calendar
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Date {
    /// Day of the month; 1-31
    pub day: i32,
    /// Month; 1-12
    pub month: i32,
    /// Year; 1-9999
    pub year: i32,
}
