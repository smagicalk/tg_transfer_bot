#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TonRevenueStatistics {
    /// A detailed statistics about Toncoins earned by the current user
    #[serde(rename(
        serialize = "tonRevenueStatistics",
        deserialize = "tonRevenueStatistics"
    ))]
    TonRevenueStatistics(crate::types::TonRevenueStatistics),
}
