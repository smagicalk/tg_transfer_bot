#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatistics {
    /// A detailed statistics about a supergroup chat
    #[serde(rename(
        serialize = "chatStatisticsSupergroup",
        deserialize = "chatStatisticsSupergroup"
    ))]
    Supergroup(crate::types::ChatStatisticsSupergroup),
    /// A detailed statistics about a channel chat
    #[serde(rename(
        serialize = "chatStatisticsChannel",
        deserialize = "chatStatisticsChannel"
    ))]
    Channel(crate::types::ChatStatisticsChannel),
}
