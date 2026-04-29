#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessAwayMessageSchedule {
    /// Send away messages always
    #[serde(rename(serialize = "businessAwayMessageScheduleAlways", deserialize = "businessAwayMessageScheduleAlways"))]
    Always,
    /// Send away messages outside of the business opening hours
    #[serde(rename(serialize = "businessAwayMessageScheduleOutsideOfOpeningHours", deserialize = "businessAwayMessageScheduleOutsideOfOpeningHours"))]
    OutsideOfOpeningHours,
    /// Send away messages only in the specified time span
    #[serde(rename(serialize = "businessAwayMessageScheduleCustom", deserialize = "businessAwayMessageScheduleCustom"))]
    Custom(crate::types::BusinessAwayMessageScheduleCustom),
}
