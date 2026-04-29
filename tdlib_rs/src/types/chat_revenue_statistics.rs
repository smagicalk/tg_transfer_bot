#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A detailed statistics about revenue earned from sponsored messages in a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueStatistics {
    /// A graph containing amount of revenue in a given hour
    pub revenue_by_hour_graph: crate::enums::StatisticalGraph,
    /// A graph containing amount of revenue
    pub revenue_graph: crate::enums::StatisticalGraph,
    /// Amount of earned revenue
    pub revenue_amount: crate::types::ChatRevenueAmount,
    /// Current conversion rate of the cryptocurrency in which revenue is calculated to USD
    pub usd_rate: f64,
}
