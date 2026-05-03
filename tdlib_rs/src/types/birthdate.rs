#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a birthdate of a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Birthdate {
    /// Day of the month; 1-31
    pub day: i32,
    /// Month of the year; 1-12
    pub month: i32,
    /// Birth year; 0 if unknown
    pub year: i32,
}
