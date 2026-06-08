#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The graph data to be asynchronously loaded through getStatisticalGraph
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StatisticalGraphAsync {
    /// The token to use for data loading
    pub token: String,
}
