#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NetworkStatistics {
    /// A full list of available network statistic entries
    #[serde(rename(serialize = "networkStatistics", deserialize = "networkStatistics"))]
    NetworkStatistics(crate::types::NetworkStatistics),
}
