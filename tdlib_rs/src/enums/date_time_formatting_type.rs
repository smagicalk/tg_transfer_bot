#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DateTimeFormattingType {
    /// The time must be shown relative to the current time ([in ] X seconds, minutes, hours, days, months, years [ago])
    #[serde(rename(serialize = "dateTimeFormattingTypeRelative", deserialize = "dateTimeFormattingTypeRelative"))]
    Relative,
    /// The date and time must be shown as absolute timestamps
    #[serde(rename(serialize = "dateTimeFormattingTypeAbsolute", deserialize = "dateTimeFormattingTypeAbsolute"))]
    Absolute(crate::types::DateTimeFormattingTypeAbsolute),
}
