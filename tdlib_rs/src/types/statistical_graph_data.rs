#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A graph data
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StatisticalGraphData {
    /// Graph data in JSON format
    pub json_data: String,
    /// If non-empty, a token which can be used to receive a zoomed in graph
    pub zoom_token: String,
}
