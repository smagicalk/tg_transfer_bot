#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageCalendarDay {
    /// Contains information about found messages sent on a specific day
    #[serde(rename(serialize = "messageCalendarDay", deserialize = "messageCalendarDay"))]
    MessageCalendarDay(crate::types::MessageCalendarDay),
}
