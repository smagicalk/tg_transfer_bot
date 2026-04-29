#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Date {
    /// Represents a date according to the Gregorian calendar
    #[serde(rename(serialize = "date", deserialize = "date"))]
    Date(crate::types::Date),
}
