#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarRevenueStatus {
    /// Contains information about Telegram Stars earned by a user or a chat
    #[serde(rename(serialize = "starRevenueStatus", deserialize = "starRevenueStatus"))]
    StarRevenueStatus(crate::types::StarRevenueStatus),
}
