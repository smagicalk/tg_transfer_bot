#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A detailed statistics about Telegram Stars earned by a user or a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarRevenueStatistics {
    /// A graph containing amount of revenue in a given day
    pub revenue_by_day_graph: crate::enums::StatisticalGraph,
    /// Telegram Star revenue status
    pub status: crate::types::StarRevenueStatus,
    /// Current conversion rate of a Telegram Star to USD
    pub usd_rate: f64,
}
