#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A full list of available network statistic entries
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NetworkStatistics {
    /// Point in time (Unix timestamp) from which the statistics are collected
    pub since_date: i32,
    /// Network statistics entries
    pub entries: Vec<crate::enums::NetworkStatisticsEntry>,
}
