#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An error message to be shown to the user instead of the graph
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StatisticalGraphError {
    /// The error message
    pub error_message: String,
}
