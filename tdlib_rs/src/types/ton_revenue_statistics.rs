#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A detailed statistics about Toncoins earned by the current user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TonRevenueStatistics {
    /// A graph containing amount of revenue in a given day
    pub revenue_by_day_graph: crate::enums::StatisticalGraph,
    /// Amount of earned revenue
    pub status: crate::types::TonRevenueStatus,
    /// Current conversion rate of nanotoncoin to USD cents
    pub usd_rate: f64,
}
