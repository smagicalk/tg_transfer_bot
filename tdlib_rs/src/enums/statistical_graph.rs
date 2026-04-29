#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StatisticalGraph {
    /// A graph data
    #[serde(rename(serialize = "statisticalGraphData", deserialize = "statisticalGraphData"))]
    Data(crate::types::StatisticalGraphData),
    /// The graph data to be asynchronously loaded through getStatisticalGraph
    #[serde(rename(serialize = "statisticalGraphAsync", deserialize = "statisticalGraphAsync"))]
    Async(crate::types::StatisticalGraphAsync),
    /// An error message to be shown to the user instead of the graph
    #[serde(rename(serialize = "statisticalGraphError", deserialize = "statisticalGraphError"))]
    Error(crate::types::StatisticalGraphError),
}
