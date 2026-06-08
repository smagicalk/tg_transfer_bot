#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessOpeningHoursInterval {
    /// Describes an interval of time when the business is open
    #[serde(rename(
        serialize = "businessOpeningHoursInterval",
        deserialize = "businessOpeningHoursInterval"
    ))]
    BusinessOpeningHoursInterval(crate::types::BusinessOpeningHoursInterval),
}
