#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains 0-based positions of matched objects
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundPositions {
    /// Total number of matched objects
    pub total_count: i32,
    /// The positions of the matched objects
    pub positions: Vec<i32>,
}
