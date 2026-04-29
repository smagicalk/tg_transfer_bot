#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageCalendar {
    /// Contains information about found messages, split by days according to the option "utc_time_offset"
    #[serde(rename(serialize = "messageCalendar", deserialize = "messageCalendar"))]
    MessageCalendar(crate::types::MessageCalendar),
}
