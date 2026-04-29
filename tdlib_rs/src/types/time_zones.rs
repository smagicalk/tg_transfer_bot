#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of time zones
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TimeZones {
    /// A list of time zones
    pub time_zones: Vec<crate::types::TimeZone>,
}
