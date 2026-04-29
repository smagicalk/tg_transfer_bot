#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DatabaseStatistics {
    /// Contains database statistics
    #[serde(rename(serialize = "databaseStatistics", deserialize = "databaseStatistics"))]
    DatabaseStatistics(crate::types::DatabaseStatistics),
}
