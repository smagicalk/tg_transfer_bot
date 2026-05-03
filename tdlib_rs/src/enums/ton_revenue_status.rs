#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TonRevenueStatus {
    /// Contains information about Toncoins earned by the current user
    #[serde(rename(serialize = "tonRevenueStatus", deserialize = "tonRevenueStatus"))]
    TonRevenueStatus(crate::types::TonRevenueStatus),
}
