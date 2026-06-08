#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarRevenueStatistics {
    /// A detailed statistics about Telegram Stars earned by a user or a chat
    #[serde(rename(
        serialize = "starRevenueStatistics",
        deserialize = "starRevenueStatistics"
    ))]
    StarRevenueStatistics(crate::types::StarRevenueStatistics),
}
