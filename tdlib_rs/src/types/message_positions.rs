#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of message positions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePositions {
    /// Total number of messages found
    pub total_count: i32,
    /// List of message positions
    pub positions: Vec<crate::types::MessagePosition>,
}
