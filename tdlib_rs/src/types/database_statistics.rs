#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains database statistics
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DatabaseStatistics {
    /// Database statistics in an unspecified human-readable format
    pub statistics: String,
}
