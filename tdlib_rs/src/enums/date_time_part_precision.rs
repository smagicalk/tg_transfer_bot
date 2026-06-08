#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DateTimePartPrecision {
    /// Don't show the date or time
    #[serde(rename(
        serialize = "dateTimePartPrecisionNone",
        deserialize = "dateTimePartPrecisionNone"
    ))]
    None,
    /// Show the date or time in a short way (17.03.22 or 22:45)
    #[serde(rename(
        serialize = "dateTimePartPrecisionShort",
        deserialize = "dateTimePartPrecisionShort"
    ))]
    Short,
    /// Show the date or time in a long way (March 17, 2022 or 22:45:00)
    #[serde(rename(
        serialize = "dateTimePartPrecisionLong",
        deserialize = "dateTimePartPrecisionLong"
    ))]
    Long,
}
